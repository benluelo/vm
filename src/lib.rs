#![feature(slice_swap_unchecked)]
// #![warn(clippy::panic, clippy::unwrap_in_result)]

use std::fmt;

use anyhow::Result;
use const_hex::ToHexExt;
use tracing::{info, trace};

pub mod assembler;
pub mod mir;

#[cfg(test)]
mod vm_tests;

pub struct Vm {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub stack: Vec<u64>,
    pub memory: Vec<u8>,
    pub cycles: u32,
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

macro_rules! try_ {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        }
    };
}

macro_rules! ok_or {
    ($e:expr, $err:expr) => {
        match $e {
            Some(t) => t,
            None => return Err($err),
        }
    };
}

macro_rules! as_ptr {
    ($e:expr) => {{
        #[cfg(target_pointer_width = "64")]
        { $e as usize }
        #[cfg(not(target_pointer_width = "64"))]
        {
            let n = $e;
            if n > (usize::MAX as u64) {
                return Error::PointerTooBig(n);
            } else {
                $e as u64
            }
        }
    }};
}

impl Vm {
    pub fn new(code: Vec<u8>, data: Vec<u8>) -> Self {
        Self {
            code,
            data,
            stack: vec![],
            memory: vec![],
            cycles: 0,
        }
    }

    pub fn run(&mut self) -> Result<Option<Vec<u8>>, Error> {
        self.run_to(None)
    }

    #[inline]
    pub fn run_to(&mut self, max_cycles: Option<u32>) -> Result<Option<Vec<u8>>, Error> {
        let mut pc = 0;

        trace!("data: {}", self.data.encode_hex());

        loop {
            match self.step(&mut pc) {
                Ok(StepResult::Stepped) => {}
                Ok(StepResult::Eof) => break Ok(None),
                Ok(StepResult::Exit(output)) => break Ok(Some(output)),
                Err(err) => {
                    info!("pc: {pc}");
                    return Err(err);
                }
            }

            self.cycles += 1;

            if let Some(x) = max_cycles
                && self.cycles > x
            {
                return Ok(None);
            }

            // std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    #[warn(clippy::question_mark_used)]
    fn step(&mut self, pc: &mut usize) -> Result<StepResult, Error> {
        #[inline(always)]
        fn u64_from_bytes(arr: &[u8]) -> u64 {
            let mut v = [0; 8];
            v[8 - arr.len()..].copy_from_slice(arr);
            u64::from_be_bytes(v)
        }

        macro_rules! pop {
            () => {{
                let len = self.stack.len();
                if len == 0 {
                    return Err(Error::StackEmpty);
                } else {
                    unsafe {
                        let x = *self.stack.get_unchecked(len - 1);
                        self.stack.set_len(len - 1);
                        x
                    }
                }
            }};
        }

        macro_rules! last {
            () => {
                ok_or!(self.stack.last_mut(), Error::StackEmpty)
            };
        }

        macro_rules! push_n {
            ($n:literal) => {{
                *pc += $n;
                let mut v = [0_u8; 8];
                let res = ok_or!(self.code.get(*pc - $n..*pc), Error::Eof);
                v[8 - $n..].copy_from_slice(res);
                let v = u64::from_be_bytes(v);
                trace!("push{} {v:x}", $n);
                self.stack.push(v);
            }};
        }

        macro_rules! write_n {
            ($n:literal) => {{
                trace!("write{}", $n);
                let value = pop!();
                let ptr = as_ptr!(pop!());
                trace!("{value:x} @ {ptr:x}");
                let bytes = value.to_be_bytes();
                ok_or!(self.memory.get_mut(ptr..ptr + $n), Error::Segfault)
                    .copy_from_slice(&bytes[8 - $n..]);
            }};
        }

        macro_rules! read_n {
            ($n:literal) => {{
                trace!("read{}", $n);
                let top = last!();
                let ptr = as_ptr!(*top);
                trace!("ptr: {ptr:x}");
                let res = ok_or!(
                    self.memory.get(ptr..((ptr + 8) - (8 - $n))),
                    Error::Segfault
                );
                *top = u64_from_bytes(res);
            }};
        }

        macro_rules! dread_n {
            ($n:literal) => {{
                trace!("dread{}", $n);
                let top = last!();
                let ptr = as_ptr!(*top);
                trace!("ptr: {ptr:x}");
                let res = ok_or!(self.data.get(ptr..ptr + $n), Error::Segfault);
                *top = u64_from_bytes(res);
            }};
        }

        macro_rules! binop {
            ($op:literal, $f:ident) => {{
                trace!($op);
                let len = self.stack.len();

                if len < 2 {
                    return Err(Error::StackEmpty);
                };

                let b = unsafe { len.unchecked_sub(2) };
                let a = unsafe { len.unchecked_sub(1) };

                unsafe {
                    *self.stack.get_unchecked_mut(b) =
                        op::$f(*self.stack.get_unchecked(b), *self.stack.get_unchecked(a))
                };

                unsafe { self.stack.set_len(a) };
            }};
        }

        if *pc >= self.code.len() {
            return Ok(StepResult::Eof);
        }
        let op = unsafe { self.code.get_unchecked(*pc) };

        *pc += 1;

        trace!("");

        match *op {
            raw::PUSH0 => {
                trace!("push0");
                self.stack.push(0);
            }
            raw::PUSH1 => push_n!(1),
            raw::PUSH2 => push_n!(2),
            raw::PUSH3 => push_n!(3),
            raw::PUSH4 => push_n!(4),
            raw::PUSH5 => push_n!(5),
            raw::PUSH6 => push_n!(6),
            raw::PUSH7 => push_n!(7),
            raw::PUSH8 => push_n!(8),
            raw::DUP => {
                trace!("dup");
                let idx = as_ptr!(*last!());
                trace!("idx = {idx:x}");
                let stack_idx = ok_or!(
                    self.stack
                        .len()
                        .checked_sub(idx)
                        .and_then(|i| i.checked_sub(2)),
                    Error::InvalidStackIdx
                );

                unsafe {
                    *self.stack.last_mut().unwrap_unchecked() =
                        *self.stack.get_unchecked(stack_idx);
                };
            }
            raw::DUP0 => {
                trace!("dup0");
                let stack_idx = ok_or!(self.stack.len().checked_sub(1), Error::InvalidStackIdx);

                self.stack
                    .push(*ok_or!(self.stack.get(stack_idx), Error::InvalidStackIdx))
            }
            raw::SWAP => {
                trace!("swap");
                let idx = as_ptr!(pop!());
                let idx = ok_or!(idx.checked_add(1), Error::InvalidStackValue);
                let len = self.stack.len();
                if len < idx {
                    return Err(Error::InvalidStackIdx);
                }
                // SAFETY: Len is at least 1 as per above
                let a_idx = unsafe { len.unchecked_sub(1) };
                // SAFETY: Len is at least idx as per above
                let b_idx = unsafe { a_idx.unchecked_sub(idx) };
                self.stack.swap(a_idx, b_idx);
            }
            raw::SWAP0 => {
                trace!("swap0");
                let b_idx = ok_or!(self.stack.len().checked_sub(2), Error::InvalidStackIdx);
                // SAFETY: Len is at least 2 as per above
                let a_idx = unsafe { self.stack.len().unchecked_sub(1) };
                self.stack.swap(a_idx, b_idx);
            }
            raw::POP => {
                trace!("pop");
                pop!();
            }
            raw::ALLOC => {
                trace!("alloc");
                let size = as_ptr!(pop!());
                self.memory.extend(vec![0; size]);
            }

            raw::WRITE1 => write_n!(1),
            raw::WRITE2 => write_n!(2),
            raw::WRITE3 => write_n!(3),
            raw::WRITE4 => write_n!(4),
            raw::WRITE5 => write_n!(5),
            raw::WRITE6 => write_n!(6),
            raw::WRITE7 => write_n!(7),
            raw::WRITE8 => write_n!(8),

            raw::READ1 => read_n!(1),
            raw::READ2 => read_n!(2),
            raw::READ3 => read_n!(3),
            raw::READ4 => read_n!(4),
            raw::READ5 => read_n!(5),
            raw::READ6 => read_n!(6),
            raw::READ7 => read_n!(7),
            raw::READ8 => read_n!(8),

            raw::DREAD1 => dread_n!(1),
            raw::DREAD2 => dread_n!(2),
            raw::DREAD3 => dread_n!(3),
            raw::DREAD4 => dread_n!(4),
            raw::DREAD5 => dread_n!(5),
            raw::DREAD6 => dread_n!(6),
            raw::DREAD7 => dread_n!(7),
            raw::DREAD8 => dread_n!(8),

            raw::DCOPY => {
                trace!("dcopy");
                if self.stack.len() < 3 {
                    return Err(Error::StackEmpty);
                };

                let [src, dst, len] = &self.stack[self.stack.len() - 3..] else {
                    unsafe { std::hint::unreachable_unchecked() }
                    // unreachable!();
                };

                let len = as_ptr!(*len);
                let dst = as_ptr!(*dst);
                let src = as_ptr!(*src);

                unsafe { self.stack.set_len(self.stack.len() - 3) };
                // self.stack.truncate(self.stack.len() - 3);

                trace!("len: {len:x}, dst: {dst:x}, src: {src:x}");

                ok_or!(self.memory.get_mut(dst..dst + len), Error::Segfault)
                    .copy_from_slice(ok_or!(self.data.get(src..src + len), Error::Segfault));
            }

            raw::DLEN => {
                trace!("dlen");
                self.stack.push(self.data.len() as u64);
            }

            raw::ADD => binop!("add", add),
            raw::SUB => binop!("sub", sub),
            raw::MUL => binop!("mul", mul),
            raw::DIV => {
                trace!("div");
                let len = self.stack.len();
                if len < 2 {
                    return Err(Error::StackEmpty);
                };
                let b = len - 2;
                let a = len - 1;
                unsafe {
                    *self.stack.get_unchecked_mut(b) = try_!(op::div(
                        *self.stack.get_unchecked(b),
                        *self.stack.get_unchecked(a)
                    ))
                };
                unsafe { self.stack.set_len(a) };
            }
            // raw::DIV => {
            //     trace!("div");
            //     let a = pop!();
            //     let b = last!();
            //     *b = op::div(a, *b)?;
            // }
            raw::EXP => binop!("exp", expmod),
            raw::MOD => {
                trace!("mod");
                let a = pop!();
                let b = last!();
                trace!("{b:x} % {a:x}");
                *b = try_!(op::r#mod(*b, a));
            }
            raw::EQ => binop!("eq", eq),
            raw::NEQ => binop!("neq", neq),
            raw::LT => binop!("lt", lt),
            raw::GT => binop!("gt", gt),
            raw::NOT => {
                trace!("not");
                let a = last!();
                *a = op::not(*a);
            }
            raw::SHR => binop!("shr", shr),
            raw::SHL => binop!("shl", shl),
            raw::NEG => {
                trace!("neg");
                let a = last!();
                *a = op::neg(*a);
            }
            raw::OR => binop!("or", or),
            raw::XOR => binop!("xor", xor),
            raw::AND => binop!("and", and),

            raw::JUMP => {
                trace!("jump");
                let dst = pop!();
                trace!("dst = {dst:x}");
                *pc = as_ptr!(dst);
            }
            raw::JNZ => {
                trace!("jnz");
                let dst = pop!();
                trace!("dst = {dst:x}");
                let value = pop!();
                trace!("value = {value:x}");
                if value != 0 {
                    *pc = as_ptr!(dst);
                }
            }
            raw::CALL => {
                trace!("call");
                let top = last!();
                let address = *top;
                *top = *pc as u64;
                *pc = as_ptr!(address);
            }
            raw::EXIT => {
                trace!("exit");
                let len = as_ptr!(pop!());
                let ptr = as_ptr!(pop!());

                return Ok(StepResult::Exit(
                    ok_or!(self.memory.get(ptr..ptr + len), Error::Segfault).to_vec(),
                ));
            }
            raw::TRAP => {
                trace!("trap");
                let value = pop!();
                return Err(Error::Trap(value));
            }
            op => return Err(Error::UnknownOp(op)),
        }

        trace!("pc: {pc:x}");
        trace!("stack: {:x?}", self.stack);
        trace!("memory: {}", self.memory.encode_hex());

        Ok(StepResult::Stepped)
    }

    #[warn(clippy::question_mark_used)]
    fn eat_op(&self, pc: &mut usize) -> Result<Option<Op>, Error> {
        #[inline(always)]
        fn push_n<const N: usize>(pc: &mut usize, code: &[u8]) -> Result<[u8; N], Error> {
            *pc += N;
            let mut v = [0; N];
            let res = try_!(code.get(*pc - N..*pc).ok_or(Error::Eof));
            v.copy_from_slice(res);
            Ok(v)
        }

        let Some(op) = self.code.get(*pc) else {
            return Ok(None);
        };

        *pc += 1;

        Ok(Some(match *op {
            raw::PUSH0 => Op::PUSH0,
            raw::PUSH1 => Op::PUSH1(try_!(push_n(pc, &self.code))),
            raw::PUSH2 => Op::PUSH2(try_!(push_n(pc, &self.code))),
            raw::PUSH3 => Op::PUSH3(try_!(push_n(pc, &self.code))),
            raw::PUSH4 => Op::PUSH4(try_!(push_n(pc, &self.code))),
            raw::PUSH5 => Op::PUSH5(try_!(push_n(pc, &self.code))),
            raw::PUSH6 => Op::PUSH6(try_!(push_n(pc, &self.code))),
            raw::PUSH7 => Op::PUSH7(try_!(push_n(pc, &self.code))),
            raw::PUSH8 => Op::PUSH8(try_!(push_n(pc, &self.code))),
            raw::DUP => Op::DUP,
            raw::DUP0 => Op::DUP0,
            raw::SWAP => Op::SWAP,
            raw::SWAP0 => Op::SWAP0,
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
            raw::SHL => Op::SHL,
            raw::SHR => Op::SHR,
            raw::NEG => Op::NEG,
            raw::OR => Op::OR,
            raw::XOR => Op::XOR,
            raw::AND => Op::AND,
            raw::JUMP => Op::JUMP,
            raw::JNZ => Op::JNZ,
            raw::CALL => Op::CALL,
            raw::EXIT => Op::EXIT,
            raw::TRAP => Op::TRAP,
            op => return Err(Error::UnknownOp(op)),
        }))
    }
}

pub mod op {
    use crate::Error;

    #[inline(always)]
    pub const fn add(a: u64, b: u64) -> u64 {
        a.wrapping_add(b)
    }

    #[inline(always)]
    pub const fn sub(a: u64, b: u64) -> u64 {
        a.wrapping_sub(b)
    }

    #[inline(always)]
    pub const fn mul(a: u64, b: u64) -> u64 {
        a.wrapping_mul(b)
    }

    #[inline(always)]
    pub const fn div(a: u64, b: u64) -> Result<u64, Error> {
        if b == 0 {
            Err(Error::DivideByZero)
        } else {
            Ok(a.wrapping_div(b))
        }
    }

    #[inline(always)]
    pub const fn not(a: u64) -> u64 {
        (a == 0) as u64
    }

    #[inline(always)]
    pub const fn gt(a: u64, b: u64) -> u64 {
        (a > b) as u64
    }

    #[inline(always)]
    pub const fn lt(a: u64, b: u64) -> u64 {
        (a < b) as u64
    }

    #[inline(always)]
    pub const fn neq(a: u64, b: u64) -> u64 {
        (a != b) as u64
    }

    #[inline(always)]
    pub const fn eq(a: u64, b: u64) -> u64 {
        (a == b) as u64
    }

    #[inline(always)]
    pub const fn r#mod(a: u64, b: u64) -> Result<u64, Error> {
        if a == 0 {
            Err(Error::DivideByZero)
        } else {
            Ok(a.wrapping_rem(b))
        }
    }

    #[inline(always)]
    pub const fn and(a: u64, b: u64) -> u64 {
        a & b
    }

    #[inline(always)]
    pub const fn xor(a: u64, b: u64) -> u64 {
        a ^ b
    }

    #[inline(always)]
    pub const fn or(a: u64, b: u64) -> u64 {
        a | b
    }

    #[inline(always)]
    pub const fn neg(a: u64) -> u64 {
        !a
    }

    #[inline]
    pub fn expmod(a: u64, b: u64) -> u64 {
        if b == 0 {
            return 1;
        }

        let mut acc: u128 = 1;
        let mut base = a as u128;
        let mut exp = b;

        loop {
            if (exp & 1) == 1 {
                acc = acc.strict_mul(base).rem_euclid(u64::MAX as _);
                // since exp!=0, finally the exp must be 1.
                if exp == 1 {
                    return acc as u64;
                }
            }
            exp >>= 1;
            base = base.strict_mul(base).rem_euclid(u64::MAX as _);
        }
    }

    pub fn shr(a: u64, shift: u64) -> u64 {
        a.unbounded_shr(shift.try_into().unwrap_or(u32::MAX))
    }

    pub fn shl(a: u64, shift: u64) -> u64 {
        a.unbounded_shl(shift.try_into().unwrap_or(u32::MAX))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn exp_ok() {
            assert_eq!(expmod(2, 24), 2_u64.pow(24));

            assert_eq!(expmod(3, 65), 7752514819847767473);

            assert_eq!(
                expmod(1844674407370955164, 18446744073709551),
                8344168819056270514
            );
        }
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
            #[cfg(doc)]
            use super::Op;

            $($(#[$meta])* pub const $Variant: u8 = $value;)+
        }
    };
}

op! {
    pub enum Op {
        // STACK OPERATIONS (0x00-0x1f)
        PUSH0 = 0x00,

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

        /// Push 8 bytes (a full word) to the stack.
        PUSH8([u8; 8]) = 0x08,

        /// Pop the item on the top of the stack as N and duplicate the Nth stack item.
        ///
        /// | Stack Input   | Stack Output  |
        /// | ------------- | ------------- |
        /// | `[..., a, N]` | `[..., a, a]` |
        DUP = 0x09,

        /// Duplicate the item on the top of the stack.
        ///
        /// | Stack Input | Stack Output  |
        /// | ------------| ------------- |
        /// | `[..., a]`  | `[..., a, a]` |
        DUP0 = 0x0a,

        /// Pop the item on the top of the stack as N and swap the first and (N-1)th stack items.
        ///
        /// | Stack Input                  | Stack Output                 |
        /// | ---------------------------- | ---------------------------- |
        /// | `[..., a, (...{n-1}), b, n]` | `[..., b, (...{n-1}), a, n]` |
        SWAP = 0x0b,

        /// Swap the top two items on the stack.
        ///
        /// | Stack Input   | Stack Output  |
        /// | ------------- | ------------- |
        /// | `[..., a, b]` | `[..., b, a]` |
        SWAP0 = 0x0c,

        /// Pop the top of the stack, returning an error if the stack is empty.
        ///
        /// | Stack Input | Stack Output |
        /// | ----------- | ------------ |
        /// | `[..., a]`  | `[...]`      |
        POP = 0x0d,

        // MEMORY OPERATIONS (0x20-0x3f)

        /// Pop the top of the stack anad grow the memory by that number of bytes.
        ///
        /// | Stack Input   | Stack Output |
        /// | ------------- | ------------ |
        /// | `[..., size]` | `[...]`      |
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
        /// Data read beyond the data length is read as zero.
        DREAD8 = 0x38,

        /// Copy a portion of data delimited by src..src+len to memory at dst.
        ///
        /// | Stack Input            | Stack Output           |
        /// | ---------------------- | ---------------------- |
        /// | `[..., src, dst, len]` | `[...]`                |
        DCOPY = 0x39,

        /// Push the length of the data to the stack.
        ///
        /// | Stack Input | Stack Output |
        /// | ----------- | ------------ |
        /// | `[...]`     | `[..., len]` |
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
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
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
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b % a]` |
        MOD = 0x45,

        /// Equality.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b == a]` |
        EQ = 0x4a,

        /// Inequality.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., b != a]` |
        NEQ = 0x4b,

        /// Less-than comparison.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b < a]` |
        LT = 0x4c,

        /// Greater-than comparison.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b > a]` |
        GT = 0x4d,

        /// Logical NOT.
        ///
        /// | Stack Input | Stack Output |
        /// | ----------- | ------------ |
        /// | `[..., a]`  | `[..., !a]`  |
        ///
        /// Note that this is not bitwise negation (see [`Op::NEG`]). The value is treated as a boolean, and value pushed back to the stack will only ever be 0 or 1.
        NOT = 0x4e,

        /// Left shift.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., a << b]` |
        ///
        /// The shift is "unbounded", and as such will always return 0 if the shift value is >= 64.
        SHL = 0x4f,

        /// Right shift.
        ///
        /// | Stack Input   | Stack Output    |
        /// | ------------- | --------------- |
        /// | `[..., b, a]` | `[..., a >> b]` |
        ///
        /// The shift is "unbounded", and as such will always return 0 if the shift value is >= 64.
        SHR = 0x50,

        /// Bitwise negation.
        ///
        /// | Stack Input | Stack Output |
        /// | ----------- | ------------ |
        /// | `[..., a]`  | `[..., ~a]`  |
        NEG = 0x51,

        /// Bitwise OR.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b | a]` |
        OR = 0x52,

        /// Bitwise XOR.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b ^ a]` |
        XOR = 0x53,

        /// Bitwise AND.
        ///
        /// | Stack Input   | Stack Output   |
        /// | ------------- | -------------- |
        /// | `[..., b, a]` | `[..., b & a]` |
        AND = 0x54,

        // CONTROL FLOW OPERATIONS (0xa0-0xaf)

        /// Pop an instruction pointer off the stack and jump to the address.
        ///
        /// | Stack Input   | Stack Output |
        /// | ------------- | ------------ |
        /// | `[..., addr]` | `[...]`      |
        JUMP = 0xa0,

        /// Pop the top value off of the stack and jump to the contained address
        /// if the value is non-zero.
        ///
        /// | Stack Input         | Stack Output |
        /// | ------------------- | ------------ |
        /// | `[..., addr, cond]` | `[...]`      |
        JNZ = 0xa1,

        /// Pop an instruction pointer off the stack, push the current address to the stack, and then jump to the previously popped address.
        ///
        /// | Stack Input   | Stack Output |
        /// | ------------- | ------------ |
        /// | `[..., addr]` | `[..., ret]` |
        CALL = 0xa2,

        /// Terminate execution with a payload. The top two values of the stack will
        /// be read as the pointer to and length of the return data.
        ///
        /// | Stack Input       | Stack Output           |
        /// | ----------------- | ---------------------- |
        /// | `[..., ptr, len]` | `<program terminates>` |
        EXIT = 0xa4,

        /// Terminate execution with an error code. The top value of the stack is
        /// used as the error code.
        ///
        /// | Stack Input   | Stack Output           |
        /// | ------------- | ---------------------- |
        /// | `[..., code]` | `<program terminates>` |
        TRAP = 0xa5,
    }
}

impl Op {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Op::PUSH0 => vec![raw::PUSH0],
            Op::PUSH1(v) => [raw::PUSH1].into_iter().chain(v).collect(),
            Op::PUSH2(v) => [raw::PUSH2].into_iter().chain(v).collect(),
            Op::PUSH3(v) => [raw::PUSH3].into_iter().chain(v).collect(),
            Op::PUSH4(v) => [raw::PUSH4].into_iter().chain(v).collect(),
            Op::PUSH5(v) => [raw::PUSH5].into_iter().chain(v).collect(),
            Op::PUSH6(v) => [raw::PUSH6].into_iter().chain(v).collect(),
            Op::PUSH7(v) => [raw::PUSH7].into_iter().chain(v).collect(),
            Op::PUSH8(v) => [raw::PUSH8].into_iter().chain(v).collect(),
            Op::DUP => vec![raw::DUP],
            Op::DUP0 => vec![raw::DUP0],
            Op::SWAP => vec![raw::SWAP],
            Op::SWAP0 => vec![raw::SWAP0],
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
            Op::SHL => vec![raw::SHL],
            Op::SHR => vec![raw::SHR],
            Op::NEG => vec![raw::NEG],
            Op::OR => vec![raw::OR],
            Op::XOR => vec![raw::XOR],
            Op::AND => vec![raw::AND],
            Op::JUMP => vec![raw::JUMP],
            Op::JNZ => vec![raw::JNZ],
            Op::CALL => vec![raw::CALL],
            Op::EXIT => vec![raw::EXIT],
            Op::TRAP => vec![raw::TRAP],
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
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

    #[cfg(not(target_pointer_width = "64"))]
    /// Attempted to use more memory than is addressable by the host system the
    /// vm was compiled for.
    #[error(
        "pointer {0} too large for host system (pointer width: {pw})",
        pw = usize::BITS,
    )]
    PointerTooBig(u64),
}
