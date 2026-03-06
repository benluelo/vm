#![warn(clippy::panic, clippy::unwrap_in_result)]
use std::{fmt, fs, path::PathBuf};

use anyhow::{Result, bail};
use argh::{FromArgValue, FromArgs};
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{Parser, error::Rich};
use const_hex::ToHexExt;
use tracing::{field::Empty, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    assembler::parse_asm,
    mir::{Ctx, compile},
};

pub mod assembler;
pub mod mir;

#[cfg(test)]
mod vm_tests;

/// Compiler and assembler.
#[derive(FromArgs, PartialEq, Debug)]
struct Args {
    /// if this flag is provided, the input file will be treated as a assembly
    /// file rather than a code file.
    #[argh(switch)]
    pub asm: bool,

    /// run the compiled object.
    #[argh(switch)]
    pub run: bool,

    /// input to be provided to the program when executing with --run.
    ///
    /// Incompatible with --input-file.
    #[argh(option)]
    pub input: Option<String>,

    /// path to the input to be provided to the program when executing with
    /// --run.
    ///
    /// Incompatible with --input.
    #[argh(option)]
    pub input_file: Option<PathBuf>,

    /// whether to treat --input as hex.
    #[argh(switch)]
    pub input_hex: bool,

    /// what to emit. defaults to object.
    #[argh(option)]
    pub emit: Emit,

    /// the file to compile.
    #[argh(positional)]
    pub file: PathBuf,

    /// the file to write the output to.
    ///
    /// If not provided, this will default to
    /// the input file name with the file extension replaced with `.o`.
    #[argh(option, short = 'o')]
    pub out: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Default, FromArgValue)]
enum Emit {
    Asm,
    #[default]
    Object,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = argh::from_env::<Args>();

    let file = fs::read_to_string(&args.file)?;

    let obj = if args.asm {
        match parse_asm().parse(&file).into_result() {
            Ok(obj) => obj.assemble(),
            Err(errs) => {
                report_errors(&file, errs);
                return Ok(());
            }
        }
    } else {
        match mir::parse::grammar().block.parse(&file).into_result() {
            Ok(obj) => {
                let mut ctx = Ctx::new_root();
                compile(&mut ctx, &obj)?;
                match args.emit {
                    Emit::Asm => {
                        let obj = ctx.into_object();
                        match args.out {
                            Some(out) => fs::write(out, obj.to_string())?,
                            None => println!("{obj}"),
                        }
                        return Ok(());
                    }
                    Emit::Object => ctx.into_object().assemble(),
                }
            }
            Err(errs) => {
                report_errors(&file, errs);
                return Ok(());
            }
        }
    };

    if args.run {
        let data = match (args.input, args.input_hex, args.input_file) {
            (None, true, None) => bail!("--input-hex requires --input"),
            (None, false, None) => vec![],
            (None, true, Some(path)) => const_hex::decode(fs::read(path)?)?,
            (None, false, Some(path)) => fs::read(path)?,
            (Some(input), true, None) => const_hex::decode(input)?,
            (Some(input), false, None) => input.into_bytes(),
            (Some(_), _, Some(_)) => {
                bail!("--input is mutually exclusive with --input-file")
            }
        };

        let mut vm = Vm::new(obj, data);
        let res = vm.run();
        match res {
            Ok(res) => match res {
                Some(res) => {
                    println!("{}", res.encode_hex());
                }
                None => {
                    println!("<no output>");
                }
            },
            Err(err) => println!("{err}"),
        }
    } else {
        let out = args.out.unwrap_or(args.file.with_extension("o"));
        fs::write(out, obj)?;
    }
    Ok(())
}

fn report_errors(file: &str, errs: Vec<Rich<'_, char>>) {
    for e in errs {
        Report::build(ReportKind::Error, ((), e.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(e.to_string())
            .with_label(
                Label::new(((), e.span().into_range()))
                    .with_message(e.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(Source::from(&file))
            .unwrap()
    }
}

pub struct Vm {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub stack: Vec<u64>,
    pub memory: Vec<u8>,
}

impl fmt::Debug for Vm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vm")
            .field("code", &self.code.encode_hex())
            .field("data", &self.data.encode_hex())
            .field("stack", &self.stack)
            .field("memory", &self.memory.encode_hex())
            .finish()
    }
}

impl Vm {
    pub fn new(code: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            code,
            data,
            stack: vec![],
            memory: vec![],
        }
    }

    pub fn run(&mut self) -> Result<Option<Vec<u8>>, Error> {
        self.run_to(None)
    }

    pub fn run_to(&mut self, max_cycles: Option<u32>) -> Result<Option<Vec<u8>>, Error> {
        let mut pc = 0;

        let mut cycles = 0;

        trace!("data: {}", self.data.encode_hex());

        loop {
            match self.step(&mut pc)? {
                StepResult::Stepped => {}
                StepResult::Eof => break Ok(None),
                StepResult::Exit(output) => break Ok(Some(output)),
            }

            cycles += 1;

            if max_cycles.is_some_and(|m| cycles > m) {
                return Ok(None);
            }

            // std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    fn step(&mut self, pc: &mut usize) -> Result<StepResult, Error> {
        fn u64_from_bytes(arr: &[u8]) -> u64 {
            let mut v = [0; 8];
            v[8 - arr.len()..].copy_from_slice(arr);
            u64::from_be_bytes(v)
        }

        let Some(op) = self.eat_op(pc)? else {
            return Ok(StepResult::Eof);
        };

        let mut pop = || self.stack.pop().ok_or(Error::StackEmpty);

        macro_rules! push_n {
            ($n:literal, $v:ident) => {{
                let v = u64_from_bytes(&$v);
                trace!("push{} {v:x}", $n);
                self.stack.push(v);
            }};
        }

        macro_rules! write_n {
            ($n:literal) => {{
                trace!("write{}", $n);
                let value = pop()?;
                let ptr = pop()? as usize;
                let bytes = value.to_be_bytes();
                self.memory
                    .get_mut(ptr..ptr + $n)
                    .ok_or(Error::Segfault)?
                    .copy_from_slice(&bytes[8 - $n..]);
                Ok(())
            }};
        }

        macro_rules! read_n {
            ($n:literal) => {{
                trace!("read{}", $n);
                let ptr = pop()? as usize;
                let res = self
                    .memory
                    .get(ptr..ptr + (8 - $n))
                    .ok_or(Error::Segfault)?;
                let value = u64_from_bytes(res);
                self.stack.push(value);
                Ok(())
            }};
        }

        macro_rules! dread_n {
            ($n:literal) => {{
                trace!("dread{}", $n);
                let ptr = pop()? as usize;
                let res = self.data.get(ptr..ptr + $n).ok_or(Error::Segfault)?;
                let value = u64_from_bytes(res);
                self.stack.push(value);
                Ok(())
            }};
        }

        trace!("");

        match op {
            Op::PUSH1(v) => push_n!(1, v),
            Op::PUSH2(v) => push_n!(2, v),
            Op::PUSH3(v) => push_n!(3, v),
            Op::PUSH4(v) => push_n!(4, v),
            Op::PUSH5(v) => push_n!(5, v),
            Op::PUSH6(v) => push_n!(6, v),
            Op::PUSH7(v) => push_n!(7, v),
            Op::PUSH8(v) => push_n!(8, v),
            Op::DUP => {
                trace!("dup");
                let idx = pop()? as usize;
                trace!("idx = {idx:x}");
                let stack_idx = self
                    .stack
                    .len()
                    .checked_sub(idx)
                    .and_then(|i| i.checked_sub(1))
                    .ok_or(Error::InvalidStackIdx)?;
                self.stack.push(
                    self.stack
                        .get(stack_idx)
                        .copied()
                        .ok_or(Error::InvalidStackIdx)?,
                );
            }
            Op::SWAP => {
                trace!("swap");
                let idx = pop()? as usize;
                if self.stack.len() < idx + 1 {
                    return Err(Error::InvalidStackIdx);
                }
                let a_idx = self.stack.len().checked_sub(1).ok_or(Error::StackEmpty)?;
                let b_idx = a_idx.checked_sub(idx + 1).ok_or(Error::InvalidStackIdx)?;
                self.stack.swap(a_idx, b_idx);
            }
            Op::POP => {
                trace!("pop");
                pop()?;
            }
            Op::ALLOC => {
                trace!("alloc");
                let size = pop()?;
                self.memory.extend(vec![0; size as usize]);
            }

            Op::WRITE1 => write_n!(1)?,
            Op::WRITE2 => write_n!(2)?,
            Op::WRITE3 => write_n!(3)?,
            Op::WRITE4 => write_n!(4)?,
            Op::WRITE5 => write_n!(5)?,
            Op::WRITE6 => write_n!(6)?,
            Op::WRITE7 => write_n!(7)?,
            Op::WRITE8 => write_n!(8)?,

            Op::READ1 => read_n!(1)?,
            Op::READ2 => read_n!(2)?,
            Op::READ3 => read_n!(3)?,
            Op::READ4 => read_n!(4)?,
            Op::READ5 => read_n!(5)?,
            Op::READ6 => read_n!(6)?,
            Op::READ7 => read_n!(7)?,
            Op::READ8 => read_n!(8)?,

            Op::DREAD1 => dread_n!(1)?,
            Op::DREAD2 => dread_n!(2)?,
            Op::DREAD3 => dread_n!(3)?,
            Op::DREAD4 => dread_n!(4)?,
            Op::DREAD5 => dread_n!(5)?,
            Op::DREAD6 => dread_n!(6)?,
            Op::DREAD7 => dread_n!(7)?,
            Op::DREAD8 => dread_n!(8)?,

            Op::DCOPY => {
                trace!("dcopy");
                let ptr = pop()? as usize;
                let len = pop()? as usize;
                self.memory
                    .get_mut(ptr..ptr + len)
                    .ok_or(Error::Segfault)?
                    .copy_from_slice(self.data.get(ptr..ptr + len).ok_or(Error::Segfault)?);
            }

            Op::DLEN => {
                trace!("dlen");
                self.stack.push(self.data.len() as u64);
            }

            Op::ADD => {
                trace!("add");
                let a = pop()?;
                let b = pop()?;
                self.stack.push(b.wrapping_add(a));
            }
            Op::SUB => {
                trace!("sub");
                let a = pop()?;
                let b = pop()?;
                self.stack.push(b.wrapping_sub(a));
            }
            Op::MUL => {
                trace!("mul");
                let a = pop()?;
                let b = pop()?;
                self.stack.push(b.wrapping_mul(a));
            }
            Op::DIV => {
                trace!("div");
                let a = pop()?;
                let b = pop()?;
                if b == 0 {
                    return Err(Error::DivideByZero);
                }
                self.stack.push(b.wrapping_div(a));
            }
            Op::EXP => {
                trace!("exp");
                let a = pop()?;
                let b = pop()?;
                trace!("{b:x} ** {a:x}");
                self.stack
                    .push(b.wrapping_pow(a.try_into().map_err(|_| Error::InvalidStackValue)?));
            }
            Op::MOD => {
                trace!(a = Empty, b = Empty, "mod");
                let a = pop()?;
                let b = pop()?;
                trace!("{b:x} % {a:x}");
                self.stack.push(b.wrapping_rem(a));
            }
            Op::EQ => {
                trace!("eq");
                let a = pop()?;
                let b = pop()?;
                self.stack.push((b == a) as u64);
            }
            Op::NEQ => {
                trace!("neq");
                let a = pop()?;
                let b = pop()?;
                self.stack.push((b != a) as u64);
            }
            Op::LT => {
                trace!("lt");
                let a = pop()?;
                let b = pop()?;
                trace!("{b:x} < {a:x}");
                self.stack.push((b < a) as u64);
            }
            Op::GT => {
                trace!("gt");
                let a = pop()?;
                let b = pop()?;
                trace!("{b:x} > {a:x}");
                self.stack.push((b > a) as u64);
            }
            Op::NOT => {
                trace!("not");
                let a = pop()?;
                self.stack.push((a == 0) as u64);
            }
            Op::JUMP => {
                trace!("jump");
                let dst = pop()?;
                trace!("dst = {dst:x}");
                *pc = dst.try_into().map_err(|_| Error::PointerTooBig(dst))?;
            }
            Op::JNZ => {
                trace!("jnz");
                let dst = pop()?;
                trace!("dst = {dst:x}");
                let value = pop()?;
                trace!("value = {value:x}");
                if value != 0 {
                    *pc = dst.try_into().map_err(|_| Error::PointerTooBig(dst))?;
                }
            }
            Op::CALL => {
                trace!("call");
                let address = pop()?;
                self.stack.push(*pc as u64);
                *pc = address
                    .try_into()
                    .map_err(|_| Error::PointerTooBig(address))?;
            }
            Op::EXIT => {
                trace!("exit");
                let len = pop()?;
                let len: usize = len.try_into().map_err(|_| Error::PointerTooBig(len))?;
                let ptr = pop()?;
                let ptr: usize = ptr.try_into().map_err(|_| Error::PointerTooBig(ptr))?;

                return Ok(StepResult::Exit(
                    self.memory
                        .get(ptr..ptr + len)
                        .ok_or(Error::Segfault)?
                        .to_vec(),
                ));
            }
            Op::TRAP => {
                trace!("trap");
                let value = pop()?;
                return Err(Error::Trap(value));
            }
        }

        trace!("pc: {pc:x}");
        trace!("stack: {:x?}", self.stack);
        trace!("memory: {}", self.memory.encode_hex());

        Ok(StepResult::Stepped)
    }

    fn eat_op(&self, pc: &mut usize) -> Result<Option<Op>, Error> {
        fn push_n<const N: usize>(pc: &mut usize, code: &[u8]) -> Result<[u8; N], Error> {
            *pc += N;
            let mut v = [0; N];
            let res = code.get(*pc - N..*pc).ok_or(Error::Eof)?;
            v.copy_from_slice(res);
            Ok(v)
        }

        let Some(op) = self.code.get(*pc) else {
            return Ok(None);
        };

        *pc += 1;

        Ok(Some(match *op {
            raw::PUSH1 => Op::PUSH1(push_n(pc, &self.code)?),
            raw::PUSH2 => Op::PUSH2(push_n(pc, &self.code)?),
            raw::PUSH3 => Op::PUSH3(push_n(pc, &self.code)?),
            raw::PUSH4 => Op::PUSH4(push_n(pc, &self.code)?),
            raw::PUSH5 => Op::PUSH5(push_n(pc, &self.code)?),
            raw::PUSH6 => Op::PUSH6(push_n(pc, &self.code)?),
            raw::PUSH7 => Op::PUSH7(push_n(pc, &self.code)?),
            raw::PUSH8 => Op::PUSH8(push_n(pc, &self.code)?),
            raw::DUP => Op::DUP,
            raw::SWAP => Op::SWAP,
            raw::POP => Op::POP,
            raw::ALLOC => Op::ALLOC,
            raw::WRITE1 => Op::WRITE1,
            raw::WRITE2 => Op::WRITE2,
            raw::WRITE3 => Op::WRITE3,
            raw::WRITE4 => Op::WRITE4,
            raw::WRITE5 => Op::WRITE5,
            raw::WRITE6 => Op::WRITE6,
            raw::WRITE7 => Op::WRITE7,
            raw::WRITE8 => Op::WRITE8,
            raw::READ1 => Op::READ1,
            raw::READ2 => Op::READ2,
            raw::READ3 => Op::READ3,
            raw::READ4 => Op::READ4,
            raw::READ5 => Op::READ5,
            raw::READ6 => Op::READ6,
            raw::READ7 => Op::READ7,
            raw::READ8 => Op::READ8,
            raw::DREAD1 => Op::DREAD1,
            raw::DREAD2 => Op::DREAD2,
            raw::DREAD3 => Op::DREAD3,
            raw::DREAD4 => Op::DREAD4,
            raw::DREAD5 => Op::DREAD5,
            raw::DREAD6 => Op::DREAD6,
            raw::DREAD7 => Op::DREAD7,
            raw::DREAD8 => Op::DREAD8,
            raw::DCOPY => Op::DCOPY,
            raw::DLEN => Op::DLEN,
            raw::ADD => Op::ADD,
            raw::SUB => Op::SUB,
            raw::MUL => Op::MUL,
            raw::DIV => Op::DIV,
            raw::EXP => Op::EXP,
            raw::MOD => Op::MOD,
            raw::EQ => Op::EQ,
            raw::NEQ => Op::NEQ,
            raw::LT => Op::LT,
            raw::GT => Op::GT,
            raw::NOT => Op::NOT,
            raw::JUMP => Op::JUMP,
            raw::JNZ => Op::JNZ,
            raw::CALL => Op::CALL,
            raw::EXIT => Op::EXIT,
            raw::TRAP => Op::TRAP,
            op => return Err(Error::UnknownOp(op)),
        }))
    }
}

pub enum StepResult {
    Stepped,
    Eof,
    Exit(Vec<u8>),
}

macro_rules! op {
    (pub enum $Op:ident {
        $($(#[$meta:meta])* $Variant:ident $(($tt:tt))* = $value:literal,)+
    }) => {
        #[derive(Debug, Clone, Copy)]
        #[repr(u8)]
        pub enum $Op {
            $($(#[$meta])* $Variant $(($tt))* = $value,)+
        }

        pub mod raw {
            $($(#[$meta])* pub const $Variant: u8 = $value;)+
        }
    };
}

op! {
    pub enum Op {
        // STACK OPERATIONS (0x01-0x1f)

        /// Push a single byte to to the stack.
        PUSH1([u8; 1]) = 0x01,

        /// Push 2 bytes to the stack.
        PUSH2([u8; 2]) = 0x02,

        /// Push 3 bytes to the stack.
        PUSH3([u8; 3]) = 0x03,

        /// Push 4 bytes to the stack.
        PUSH4([u8; 4]) = 0x04,

        /// Push 5 bytes to the stack.
        PUSH5([u8; 5]) = 0x05,

        /// Push 6 bytes to the stack.
        PUSH6([u8; 6]) = 0x06,

        /// Push 7 bytes to the stack.
        PUSH7([u8; 7]) = 0x07,

        /// Push 8 bytes (full word) to the stack.
        PUSH8([u8; 8]) = 0x08,

        /// Pop the item on the top of the stack as N and duplicate the Nth stack item.
        DUP = 0x09,

        /// Pop the item on the top of the stack as N and swap the first and Nth stack items.
        SWAP = 0x0a,

        /// Pop the top of the stack, returning an error if the stack is empty.
        POP = 0x0b,

        // MEMORY OPERATIONS (0x20-0x3f)

        /// Grow the memory by the value on the top of the stack specified number of
        /// bytes.
        ALLOC = 0x20,

        WRITE1 = 0x21,

        WRITE2 = 0x22,

        WRITE3 = 0x23,

        WRITE4 = 0x24,

        WRITE5 = 0x25,

        WRITE6 = 0x26,

        WRITE7 = 0x27,

        /// Write the full value on the top of the stack at the memory location of the
        /// 2nd top item in the stack.
        WRITE8 = 0x28,

        READ1 = 0x29,

        READ2 = 0x2a,

        READ3 = 0x2b,

        READ4 = 0x2c,

        READ5 = 0x2d,

        READ6 = 0x2e,

        READ7 = 0x2f,

        /// Read the full value at the memory location specified by the top value on the
        /// stack to the top of the stack.
        ///
        /// Unwritten memory is read as zero.
        READ8 = 0x30,

        DREAD1 = 0x31,

        DREAD2 = 0x32,

        DREAD3 = 0x33,

        DREAD4 = 0x34,

        DREAD5 = 0x35,

        DREAD6 = 0x36,

        DREAD7 = 0x37,

        /// Read the full value at the memory location specified by the top value on the
        /// stack to the top of the stack.
        ///
        /// Unwritten memory is read as zero.
        DREAD8 = 0x38,

        DCOPY = 0x39,
        DLEN = 0x3a,

        // ARITHMETIC OPERATIONS (0x40-0x4f)

        /// Wrapping addition.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b + a]` |
        ADD = 0x40,

        /// Wrapping subtraction.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b - a]` |
        SUB = 0x41,

        /// Wrapping multiplication.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b * a]` |
        MUL = 0x42,

        /// Floor division.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b // a]` |
        ///
        /// This operation will return an error if the divisor is zero.
        DIV = 0x43,

        /// Wrapping exponentiation.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b ** a]` |
        EXP = 0x44,

        /// Modulus (remainder).
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b % a]` |
        MOD = 0x45,

        /// Equality.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b == a]` |
        EQ = 0x4a,

        // TODO: Remove in favour of NOT opcode?
        NEQ = 0x4b,

        /// Less-than comparison.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b < a]` |
        LT = 0x4c,

        /// Greater-than comparison.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b > a]` |
        GT = 0x4d,

        /// Logical NOT.
        ///
        /// | Stack Input | Stack Output |
        /// | ----------- | ------------ |
        /// | `[..., a]`  | `[..., !a]`  |
        ///
        /// Note that this is not bitwise negation (see [`Op::BNOT`]). The value is treated as a boolean, and value pushed back to the stack will only ever be 0 or 1.
        NOT = 0x4e,

        // CONTROL FLOW OPERATIONS (0x50-0x5a)

        /// Pop an instruction pointer off the stack and jump to the address.
        JUMP = 0x50,

        /// Pop the top value off of the stack and jump to the contained address
        /// if the value is non-zero.
        JNZ = 0x51,

        /// Pop an instruction pointer off the stack, push the current address to the stack, and then jump to the previously popped address.
        CALL = 0x52,

        /// Terminate execution with a payload. The top two values of the stack will
        /// be read as the pointer to and length of the return data.
        EXIT = 0x54,

        /// Terminate execution with an error code. The top value of the stack is
        /// used as the error code.
        TRAP = 0x55,
    }
}

impl Op {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Op::PUSH1(v) => [raw::PUSH1].into_iter().chain(v).collect(),
            Op::PUSH2(v) => [raw::PUSH2].into_iter().chain(v).collect(),
            Op::PUSH3(v) => [raw::PUSH3].into_iter().chain(v).collect(),
            Op::PUSH4(v) => [raw::PUSH4].into_iter().chain(v).collect(),
            Op::PUSH5(v) => [raw::PUSH5].into_iter().chain(v).collect(),
            Op::PUSH6(v) => [raw::PUSH6].into_iter().chain(v).collect(),
            Op::PUSH7(v) => [raw::PUSH7].into_iter().chain(v).collect(),
            Op::PUSH8(v) => [raw::PUSH8].into_iter().chain(v).collect(),
            Op::DUP => vec![raw::DUP],
            Op::SWAP => vec![raw::SWAP],
            Op::POP => vec![raw::POP],
            Op::ALLOC => vec![raw::ALLOC],
            Op::WRITE1 => vec![raw::WRITE1],
            Op::WRITE2 => vec![raw::WRITE2],
            Op::WRITE3 => vec![raw::WRITE3],
            Op::WRITE4 => vec![raw::WRITE4],
            Op::WRITE5 => vec![raw::WRITE5],
            Op::WRITE6 => vec![raw::WRITE6],
            Op::WRITE7 => vec![raw::WRITE7],
            Op::WRITE8 => vec![raw::WRITE8],
            Op::READ1 => vec![raw::READ1],
            Op::READ2 => vec![raw::READ2],
            Op::READ3 => vec![raw::READ3],
            Op::READ4 => vec![raw::READ4],
            Op::READ5 => vec![raw::READ5],
            Op::READ6 => vec![raw::READ6],
            Op::READ7 => vec![raw::READ7],
            Op::READ8 => vec![raw::READ8],
            Op::DREAD1 => vec![raw::DREAD1],
            Op::DREAD2 => vec![raw::DREAD2],
            Op::DREAD3 => vec![raw::DREAD3],
            Op::DREAD4 => vec![raw::DREAD4],
            Op::DREAD5 => vec![raw::DREAD5],
            Op::DREAD6 => vec![raw::DREAD6],
            Op::DREAD7 => vec![raw::DREAD7],
            Op::DREAD8 => vec![raw::DREAD8],
            Op::DCOPY => vec![raw::DCOPY],
            Op::DLEN => vec![raw::DLEN],
            Op::ADD => vec![raw::ADD],
            Op::SUB => vec![raw::SUB],
            Op::MUL => vec![raw::MUL],
            Op::DIV => vec![raw::DIV],
            Op::EXP => vec![raw::EXP],
            Op::MOD => vec![raw::MOD],
            Op::EQ => vec![raw::EQ],
            Op::NEQ => vec![raw::NEQ],
            Op::LT => vec![raw::LT],
            Op::GT => vec![raw::GT],
            Op::NOT => vec![raw::NOT],
            Op::JUMP => vec![raw::JUMP],
            Op::JNZ => vec![raw::JNZ],
            Op::CALL => vec![raw::CALL],
            Op::EXIT => vec![raw::EXIT],
            Op::TRAP => vec![raw::TRAP],
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Attempted to pop off of an empty stack.
    #[error("stack empty")]
    StackEmpty,
    /// Attempted to read a stack index that doesn't exist.
    #[error("invalid stack idx")]
    InvalidStackIdx,
    /// Attempted to read past the max allocated memory address.
    #[error("segfault")]
    Segfault,
    /// Unexpected EOF when executing code.
    #[error("eof")]
    Eof,
    /// Attempted to divide by zero.
    #[error("divide by zero")]
    DivideByZero,
    /// Invalid stack value for operation.
    #[error("invalid stack value")]
    InvalidStackValue,

    /// Trap opcode was executed.
    #[error("trap: {0:#x}")]
    Trap(u64),
    /// Unknown operand.
    #[error("unknown op: {0:#x}")]
    UnknownOp(u8),

    /// Attempted to use more memory than is addressable by the host system the
    /// vm was compiled for.
    #[error(
        "pointer {0} too large for host system (pointer width: {pw})",
        pw = usize::BITS,
    )]
    PointerTooBig(u64),
}
