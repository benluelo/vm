use std::{
    borrow::Cow,
    collections::BTreeMap,
    fmt::{self, Display},
};

use chumsky::{
    prelude::*,
    text::{
        ascii::{ident, keyword},
        newline,
    },
};
use indexmap::IndexMap;
use tracing::{info, trace};

use crate::Op;

#[derive(Debug, Clone, PartialEq)]
pub struct Object<'a>(pub IndexMap<Cow<'a, str>, Vec<AsmOp<'a>>>);

impl<'a> Display for Object<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, ops) in &self.0 {
            writeln!(f, ":{k}")?;
            for op in ops {
                writeln!(f, "\t{op}")?;
            }
        }

        Ok(())
    }
}

impl<'a> Object<'a> {
    pub fn from_ops(ops: Vec<AsmOp<'a>>) -> Self {
        Self([("".into(), ops)].into_iter().collect())
    }

    pub fn assemble(mut self) -> Vec<u8> {
        let mut out = vec![];

        // jump threading
        loop {
            let mut changed = false;
            let mut it = self.0.iter_mut().peekable();
            while let Some((label, ops)) = it.next() {
                if let [.., AsmOp::PUSHL(dst), AsmOp::JUMP] = &**ops
                    && let Some((next_label, _)) = it.peek()
                    && dst == *next_label
                {
                    info!("threading {label} into {next_label}");
                    changed = true;
                    ops.pop();
                    ops.pop();
                }
            }
            if !changed {
                break;
            }
        }

        // replace duplicate push ops with a dup
        loop {
            let mut changed = false;
            for (_, x) in &mut self.0 {
                let mut new_ops = Vec::with_capacity(x.len());
                let mut it = x.iter().peekable();
                while let Some(a) = it.next() {
                    match a {
                        // deduping push1 doesn't make a binary size difference and just costs more
                        // cycles
                        // only worth it if there is a sequence of more than 2 AsmOp::PUSH1(_)
                        AsmOp::PUSH1(_)
                        | AsmOp::PUSH2(_)
                        | AsmOp::PUSH3(_)
                        | AsmOp::PUSH4(_)
                        | AsmOp::PUSH5(_)
                        | AsmOp::PUSH6(_)
                        | AsmOp::PUSH7(_)
                        | AsmOp::PUSH8(_) => {
                            new_ops.push(a.clone());
                            while it.next_if_eq(a).is_some() {
                                info!("deduplicating {a}");
                                changed = true;
                                // new_ops.push(AsmOp::PUSH0);
                                new_ops.push(AsmOp::DUP0);
                            }
                        }
                        // AsmOp::PUSHL(cow) => todo!(),
                        _ => {
                            new_ops.push(a.clone());
                        }
                    }
                }
                *x = new_ops;
            }
            if !changed {
                break;
            }
        }

        let max_ptr_8 = self.label_ptrs(8).iter().map(|x| *x.1).max().unwrap();
        let max_ptr_size_8 = bytes_needed_for_number(max_ptr_8);
        info!("max ptr with ptr size 8 is {max_ptr_8}, width {max_ptr_size_8}");

        let label_ptrs = self.label_ptrs(max_ptr_size_8 as usize);

        let max_ptr = self.label_ptrs(max_ptr_size_8 as usize).iter().map(|x| *x.1).max().unwrap();
        let max_ptr_size = bytes_needed_for_number(max_ptr);
        info!("max ptr with size {max_ptr_size} is {max_ptr}, width {max_ptr_size}");

        trace!("label_ptrs: {label_ptrs:x?}");

        for (_, asm) in &self.0 {
            for op in asm {
                let op = match op {
                    AsmOp::PUSH0 => Op::PUSH0,
                    AsmOp::PUSH1(v) => Op::PUSH1(*v),
                    AsmOp::PUSH2(v) => Op::PUSH2(*v),
                    AsmOp::PUSH3(v) => Op::PUSH3(*v),
                    AsmOp::PUSH4(v) => Op::PUSH4(*v),
                    AsmOp::PUSH5(v) => Op::PUSH5(*v),
                    AsmOp::PUSH6(v) => Op::PUSH6(*v),
                    AsmOp::PUSH7(v) => Op::PUSH7(*v),
                    AsmOp::PUSH8(v) => Op::PUSH8(*v),
                    AsmOp::PUSHL(label) => match max_ptr_size {
                        1 => Op::PUSH1(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        2 => Op::PUSH2(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        3 => Op::PUSH3(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        4 => Op::PUSH4(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        5 => Op::PUSH5(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        6 => Op::PUSH6(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        7 => Op::PUSH7(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        8 => Op::PUSH8(*label_ptrs[&**label].to_be_bytes().rsplit_array_ref().1),
                        _ => unreachable!(),
                    },
                    AsmOp::DUP => Op::DUP,
                    AsmOp::DUP0 => Op::DUP0,
                    AsmOp::SWAP => Op::SWAP,
                    AsmOp::SWAP0 => Op::SWAP0,
                    AsmOp::POP => Op::POP,
                    AsmOp::ALLOC => Op::ALLOC,
                    AsmOp::WRITE1 => Op::WRITE1,
                    AsmOp::WRITE2 => Op::WRITE2,
                    AsmOp::WRITE3 => Op::WRITE3,
                    AsmOp::WRITE4 => Op::WRITE4,
                    AsmOp::WRITE5 => Op::WRITE5,
                    AsmOp::WRITE6 => Op::WRITE6,
                    AsmOp::WRITE7 => Op::WRITE7,
                    AsmOp::WRITE8 => Op::WRITE8,
                    AsmOp::READ1 => Op::READ1,
                    AsmOp::READ2 => Op::READ2,
                    AsmOp::READ3 => Op::READ3,
                    AsmOp::READ4 => Op::READ4,
                    AsmOp::READ5 => Op::READ5,
                    AsmOp::READ6 => Op::READ6,
                    AsmOp::READ7 => Op::READ7,
                    AsmOp::READ8 => Op::READ8,
                    AsmOp::DREAD1 => Op::DREAD1,
                    AsmOp::DREAD2 => Op::DREAD2,
                    AsmOp::DREAD3 => Op::DREAD3,
                    AsmOp::DREAD4 => Op::DREAD4,
                    AsmOp::DREAD5 => Op::DREAD5,
                    AsmOp::DREAD6 => Op::DREAD6,
                    AsmOp::DREAD7 => Op::DREAD7,
                    AsmOp::DREAD8 => Op::DREAD8,
                    AsmOp::DCOPY => Op::DCOPY,
                    AsmOp::DLEN => Op::DLEN,
                    AsmOp::ADD => Op::ADD,
                    AsmOp::SUB => Op::SUB,
                    AsmOp::MUL => Op::MUL,
                    AsmOp::DIV => Op::DIV,
                    AsmOp::EXP => Op::EXP,
                    AsmOp::MOD => Op::MOD,
                    AsmOp::EQ => Op::EQ,
                    AsmOp::NEQ => Op::NEQ,
                    AsmOp::LT => Op::LT,
                    AsmOp::GT => Op::GT,
                    AsmOp::NOT => Op::NOT,
                    AsmOp::SHL => Op::SHL,
                    AsmOp::SHR => Op::SHR,
                    AsmOp::NEG => Op::NEG,
                    AsmOp::OR => Op::OR,
                    AsmOp::XOR => Op::XOR,
                    AsmOp::AND => Op::AND,
                    AsmOp::JUMP => Op::JUMP,
                    AsmOp::JNZ => Op::JNZ,
                    AsmOp::CALL => Op::CALL,
                    AsmOp::EXIT => Op::EXIT,
                    AsmOp::TRAP => Op::TRAP,
                };

                out.extend(op.to_bytes());
            }
        }

        out
    }

    fn label_ptrs(&self, label_ptr_size: usize) -> BTreeMap<Cow<'a, str>, u64> {
        self.0
            .iter()
            .map(|(label, asm)| {
                (label.clone(), asm.iter().map(|op| op.size(label_ptr_size)).sum::<usize>())
            })
            .collect::<Vec<_>>()
            .iter()
            .scan(0, |acc, (label, size)| {
                let ptr = *acc as u64;
                *acc += size;
                Some((label.clone(), ptr))
            })
            .collect::<BTreeMap<_, _>>()
    }
}

#[derive(Clone, PartialEq)]
pub enum AsmOp<'a> {
    PUSH0,
    PUSH1([u8; 1]),
    PUSH2([u8; 2]),
    PUSH3([u8; 3]),
    PUSH4([u8; 4]),
    PUSH5([u8; 5]),
    PUSH6([u8; 6]),
    PUSH7([u8; 7]),
    PUSH8([u8; 8]),
    PUSHL(Cow<'a, str>),
    DUP,
    DUP0,
    SWAP,
    SWAP0,
    POP,
    ALLOC,
    WRITE1,
    WRITE2,
    WRITE3,
    WRITE4,
    WRITE5,
    WRITE6,
    WRITE7,
    WRITE8,
    READ1,
    READ2,
    READ3,
    READ4,
    READ5,
    READ6,
    READ7,
    READ8,
    DREAD1,
    DREAD2,
    DREAD3,
    DREAD4,
    DREAD5,
    DREAD6,
    DREAD7,
    DREAD8,
    DCOPY,
    DLEN,
    ADD,
    SUB,
    MUL,
    DIV,
    EXP,
    MOD,
    EQ,
    NEQ,
    LT,
    GT,
    NOT,
    SHL,
    SHR,
    NEG,
    OR,
    XOR,
    AND,
    JUMP,
    JNZ,
    CALL,
    EXIT,
    TRAP,
}

impl PartialEq<&AsmOp<'_>> for AsmOp<'_> {
    fn eq(&self, other: &&AsmOp) -> bool {
        self == *other
    }
}

impl PartialEq<AsmOp<'_>> for &AsmOp<'_> {
    fn eq(&self, other: &AsmOp) -> bool {
        *self == other
    }
}

impl<'a> AsmOp<'a> {
    pub fn push(n: u64) -> Self {
        let bz = n.to_be_bytes();

        match bz {
            [0, 0, 0, 0, 0, 0, 0, 0] => Self::PUSH0,
            [0, 0, 0, 0, 0, 0, 0, ..] => Self::PUSH1(bz[7..].try_into().unwrap()),
            [0, 0, 0, 0, 0, 0, ..] => Self::PUSH2(bz[6..].try_into().unwrap()),
            [0, 0, 0, 0, 0, ..] => Self::PUSH3(bz[5..].try_into().unwrap()),
            [0, 0, 0, 0, ..] => Self::PUSH4(bz[4..].try_into().unwrap()),
            [0, 0, 0, ..] => Self::PUSH5(bz[3..].try_into().unwrap()),
            [0, 0, ..] => Self::PUSH6(bz[2..].try_into().unwrap()),
            [0, ..] => Self::PUSH7(bz[1..].try_into().unwrap()),
            [..] => Self::PUSH8(bz),
        }
    }
}

impl<'a> Display for AsmOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsmOp::PUSH0 => write!(f, "push0"),
            AsmOp::PUSH1([a]) => write!(f, "push1 0x{a:0>2x}"),
            AsmOp::PUSH2([a, b]) => write!(f, "push2 0x{a:0>2x}{b:0>2x}"),
            AsmOp::PUSH3([a, b, c]) => write!(f, "push3 0x{a:0>2x}{b:0>2x}{c:0>2x}"),
            AsmOp::PUSH4([a, b, c, d]) => write!(f, "push4 0x{a:0>2x}{b:0>2x}{c:0>2x}{d:0>2x}"),
            AsmOp::PUSH5([a, b, c, d, e]) => {
                write!(f, "push5 0x{a:0>2x}{b:0>2x}{c:0>2x}{d:0>2x}{e:0>2x}")
            }
            AsmOp::PUSH6([a, b, c, d, e, g]) => {
                write!(f, "push6 0x{a:0>2x}{b:0>2x}{c:0>2x}{d:0>2x}{e:0>2x}{g:0>2x}")
            }
            AsmOp::PUSH7([a, b, c, d, e, g, h]) => {
                write!(f, "push7 0x{a:0>2x}{b:0>2x}{c:0>2x}{d:0>2x}{e:0>2x}{g:0>2x}{h:0>2x}")
            }
            AsmOp::PUSH8([a, b, c, d, e, g, h, i]) => {
                write!(
                    f,
                    "push8 0x{a:0>2x}{b:0>2x}{c:0>2x}{d:0>2x}{e:0>2x}{g:0>2x}{h:0>2x}{i:0>2x}"
                )
            }
            AsmOp::PUSHL(label) => write!(f, "pushl @{label}"),
            AsmOp::DUP => write!(f, "dup"),
            AsmOp::DUP0 => write!(f, "dup0"),
            AsmOp::SWAP => write!(f, "swap"),
            AsmOp::SWAP0 => write!(f, "swap0"),
            AsmOp::POP => write!(f, "pop"),
            AsmOp::ALLOC => write!(f, "alloc"),
            AsmOp::WRITE1 => write!(f, "write1"),
            AsmOp::WRITE2 => write!(f, "write2"),
            AsmOp::WRITE3 => write!(f, "write3"),
            AsmOp::WRITE4 => write!(f, "write4"),
            AsmOp::WRITE5 => write!(f, "write5"),
            AsmOp::WRITE6 => write!(f, "write6"),
            AsmOp::WRITE7 => write!(f, "write7"),
            AsmOp::WRITE8 => write!(f, "write8"),
            AsmOp::READ1 => write!(f, "read1"),
            AsmOp::READ2 => write!(f, "read2"),
            AsmOp::READ3 => write!(f, "read3"),
            AsmOp::READ4 => write!(f, "read4"),
            AsmOp::READ5 => write!(f, "read5"),
            AsmOp::READ6 => write!(f, "read6"),
            AsmOp::READ7 => write!(f, "read7"),
            AsmOp::READ8 => write!(f, "read8"),
            AsmOp::DREAD1 => write!(f, "dread1"),
            AsmOp::DREAD2 => write!(f, "dread2"),
            AsmOp::DREAD3 => write!(f, "dread3"),
            AsmOp::DREAD4 => write!(f, "dread4"),
            AsmOp::DREAD5 => write!(f, "dread5"),
            AsmOp::DREAD6 => write!(f, "dread6"),
            AsmOp::DREAD7 => write!(f, "dread7"),
            AsmOp::DREAD8 => write!(f, "dread8"),
            AsmOp::DCOPY => write!(f, "dcopy"),
            AsmOp::DLEN => write!(f, "dlen"),
            AsmOp::ADD => write!(f, "add"),
            AsmOp::SUB => write!(f, "sub"),
            AsmOp::MUL => write!(f, "mul"),
            AsmOp::DIV => write!(f, "div"),
            AsmOp::EXP => write!(f, "exp"),
            AsmOp::MOD => write!(f, "mod"),
            AsmOp::EQ => write!(f, "eq"),
            AsmOp::NEQ => write!(f, "neq"),
            AsmOp::LT => write!(f, "lt"),
            AsmOp::GT => write!(f, "gt"),
            AsmOp::NOT => write!(f, "not"),
            AsmOp::SHL => write!(f, "shl"),
            AsmOp::SHR => write!(f, "shr"),
            AsmOp::NEG => write!(f, "neg"),
            AsmOp::OR => write!(f, "or"),
            AsmOp::XOR => write!(f, "xor"),
            AsmOp::AND => write!(f, "and"),
            AsmOp::JUMP => write!(f, "jump"),
            AsmOp::JNZ => write!(f, "jnz"),
            AsmOp::CALL => write!(f, "call"),
            AsmOp::EXIT => write!(f, "exit"),
            AsmOp::TRAP => write!(f, "trap"),
        }
    }
}

impl<'a> fmt::Debug for AsmOp<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PUSH0 => f.write_fmt(format_args!("PUSH0")),
            Self::PUSH1([n]) => f.write_fmt(format_args!("PUSH1({n})")),
            Self::PUSH2(n) => f.debug_tuple("PUSH2").field(n).finish(),
            Self::PUSH3(n) => f.debug_tuple("PUSH3").field(n).finish(),
            Self::PUSH4(n) => f.debug_tuple("PUSH4").field(n).finish(),
            Self::PUSH5(n) => f.debug_tuple("PUSH5").field(n).finish(),
            Self::PUSH6(n) => f.debug_tuple("PUSH6").field(n).finish(),
            Self::PUSH7(n) => f.debug_tuple("PUSH7").field(n).finish(),
            Self::PUSH8(n) => f.write_fmt(format_args!("PUSH8({})", u64::from_be_bytes(*n))),
            Self::PUSHL(label) => f.debug_tuple("PUSHL").field(label).finish(),
            Self::DUP => write!(f, "DUP"),
            Self::DUP0 => write!(f, "DUP0"),
            Self::SWAP => write!(f, "SWAP"),
            Self::SWAP0 => write!(f, "SWAP0"),
            Self::POP => write!(f, "POP"),
            Self::ALLOC => write!(f, "ALLOC"),
            Self::WRITE1 => write!(f, "WRITE1"),
            Self::WRITE2 => write!(f, "WRITE2"),
            Self::WRITE3 => write!(f, "WRITE3"),
            Self::WRITE4 => write!(f, "WRITE4"),
            Self::WRITE5 => write!(f, "WRITE5"),
            Self::WRITE6 => write!(f, "WRITE6"),
            Self::WRITE7 => write!(f, "WRITE7"),
            Self::WRITE8 => write!(f, "WRITE8"),
            Self::READ1 => write!(f, "READ1"),
            Self::READ2 => write!(f, "READ2"),
            Self::READ3 => write!(f, "READ3"),
            Self::READ4 => write!(f, "READ4"),
            Self::READ5 => write!(f, "READ5"),
            Self::READ6 => write!(f, "READ6"),
            Self::READ7 => write!(f, "READ7"),
            Self::READ8 => write!(f, "READ8"),
            Self::DREAD1 => write!(f, "DREAD1"),
            Self::DREAD2 => write!(f, "DREAD2"),
            Self::DREAD3 => write!(f, "DREAD3"),
            Self::DREAD4 => write!(f, "DREAD4"),
            Self::DREAD5 => write!(f, "DREAD5"),
            Self::DREAD6 => write!(f, "DREAD6"),
            Self::DREAD7 => write!(f, "DREAD7"),
            Self::DREAD8 => write!(f, "DREAD8"),
            Self::DCOPY => write!(f, "DCOPY"),
            Self::DLEN => write!(f, "DLEN"),
            Self::ADD => write!(f, "ADD"),
            Self::SUB => write!(f, "SUB"),
            Self::MUL => write!(f, "MUL"),
            Self::DIV => write!(f, "DIV"),
            Self::EXP => write!(f, "EXP"),
            Self::MOD => write!(f, "MOD"),
            Self::EQ => write!(f, "EQ"),
            Self::NEQ => write!(f, "NEQ"),
            Self::LT => write!(f, "LT"),
            Self::GT => write!(f, "GT"),
            Self::NOT => write!(f, "NOT"),
            Self::SHL => write!(f, "SHL"),
            Self::SHR => write!(f, "SHR"),
            Self::NEG => write!(f, "NEG"),
            Self::OR => write!(f, "OR"),
            Self::XOR => write!(f, "XOR"),
            Self::AND => write!(f, "AND"),
            Self::JUMP => write!(f, "JUMP"),
            Self::JNZ => write!(f, "JNZ"),
            Self::CALL => write!(f, "CALL"),
            Self::EXIT => write!(f, "EXIT"),
            Self::TRAP => write!(f, "TRAP"),
        }
    }
}

impl<'a> AsmOp<'a> {
    pub fn size(&self, label_ptr_size: usize) -> usize {
        match self {
            AsmOp::PUSH1(_) => 2,
            AsmOp::PUSH2(_) => 3,
            AsmOp::PUSH3(_) => 4,
            AsmOp::PUSH4(_) => 5,
            AsmOp::PUSH5(_) => 6,
            AsmOp::PUSH6(_) => 7,
            AsmOp::PUSH7(_) => 8,
            AsmOp::PUSH8(_) => 9,
            AsmOp::PUSHL(_) => 1 + label_ptr_size,
            _ => 1,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LitOrRef<'a, T> {
    Lit(T),
    Ref(&'a str),
}

fn ref_<'a>() -> impl Parser<'a, &'a str, &'a str, extra::Err<Rich<'a, char>>> {
    just('@').ignore_then(ident()).padded()
}

fn comment<'a>() -> impl Parser<'a, &'a str, (), extra::Err<Rich<'a, char>>> {
    just(";").then(any().and_is(newline().not()).repeated()).padded().ignored()
}

fn lit<'a, const N: usize>() -> impl Parser<'a, &'a str, [u8; N], extra::Err<Rich<'a, char>>> {
    just("0x")
        .ignore_then(
            one_of("0123456789abcdefABCDEF")
                .map(|c: char| match c.to_ascii_lowercase() {
                    c @ '0'..='9' => c as u8 - b'0',
                    c @ 'a'..='f' => c as u8 - 87,
                    _ => unreachable!(),
                })
                .repeated()
                .collect_exactly::<[u8; 2]>()
                .map(|[hi, lo]| (hi << 4) + lo)
                .repeated()
                .collect_exactly::<[u8; N]>(),
        )
        .padded()
}

fn parse_op<'a>() -> impl Parser<'a, &'a str, AsmOp<'a>, extra::Err<Rich<'a, char>>> {
    choice((
        choice((
            keyword("push0").padded().to(AsmOp::PUSH0),
            keyword("push1").padded().ignore_then(lit().map(AsmOp::PUSH1)),
            keyword("push2").padded().ignore_then(lit().map(AsmOp::PUSH2)),
            keyword("push3").padded().ignore_then(lit().map(AsmOp::PUSH3)),
            keyword("push4").padded().ignore_then(lit().map(AsmOp::PUSH4)),
            keyword("push5").padded().ignore_then(lit().map(AsmOp::PUSH5)),
            keyword("push6").padded().ignore_then(lit().map(AsmOp::PUSH6)),
            keyword("push7").padded().ignore_then(lit().map(AsmOp::PUSH7)),
            keyword("push8").padded().ignore_then(lit().map(AsmOp::PUSH8)),
            keyword("pushl").padded().ignore_then(ref_().map(|s| AsmOp::PUSHL(s.into()))),
        )),
        keyword("dup").padded().to(AsmOp::DUP),
        keyword("swap").padded().to(AsmOp::SWAP),
        keyword("pop").padded().to(AsmOp::POP),
        keyword("alloc").padded().to(AsmOp::ALLOC),
        choice((
            keyword("write1").padded().to(AsmOp::WRITE1),
            keyword("write2").padded().to(AsmOp::WRITE2),
            keyword("write3").padded().to(AsmOp::WRITE3),
            keyword("write4").padded().to(AsmOp::WRITE4),
            keyword("write5").padded().to(AsmOp::WRITE5),
            keyword("write6").padded().to(AsmOp::WRITE6),
            keyword("write7").padded().to(AsmOp::WRITE7),
            keyword("write8").padded().to(AsmOp::WRITE8),
        )),
        choice((
            keyword("read1").padded().to(AsmOp::READ1),
            keyword("read2").padded().to(AsmOp::READ2),
            keyword("read3").padded().to(AsmOp::READ3),
            keyword("read4").padded().to(AsmOp::READ4),
            keyword("read5").padded().to(AsmOp::READ5),
            keyword("read6").padded().to(AsmOp::READ6),
            keyword("read7").padded().to(AsmOp::READ7),
            keyword("read8").padded().to(AsmOp::READ8),
        )),
        choice((
            keyword("dread1").padded().to(AsmOp::DREAD1),
            keyword("dread2").padded().to(AsmOp::DREAD2),
            keyword("dread3").padded().to(AsmOp::DREAD3),
            keyword("dread4").padded().to(AsmOp::DREAD4),
            keyword("dread5").padded().to(AsmOp::DREAD5),
            keyword("dread6").padded().to(AsmOp::DREAD6),
            keyword("dread7").padded().to(AsmOp::DREAD7),
            keyword("dread8").padded().to(AsmOp::DREAD8),
            keyword("dcopy").padded().to(AsmOp::DCOPY),
            keyword("dlen").padded().to(AsmOp::DLEN),
        )),
        choice((
            keyword("add").padded().to(AsmOp::ADD),
            keyword("sub").padded().to(AsmOp::SUB),
            keyword("mul").padded().to(AsmOp::MUL),
            keyword("div").padded().to(AsmOp::DIV),
            keyword("exp").padded().to(AsmOp::EXP),
            keyword("mod").padded().to(AsmOp::MOD),
            keyword("eq").padded().to(AsmOp::EQ),
            keyword("neq").padded().to(AsmOp::NEQ),
            keyword("lt").padded().to(AsmOp::LT),
            keyword("gt").padded().to(AsmOp::GT),
            keyword("not").padded().to(AsmOp::NOT),
            keyword("shl").padded().to(AsmOp::SHL),
            keyword("shr").padded().to(AsmOp::SHR),
            keyword("neg").padded().to(AsmOp::NEG),
            keyword("or").padded().to(AsmOp::OR),
            keyword("and").padded().to(AsmOp::AND),
        )),
        keyword("jnz").padded().to(AsmOp::JNZ),
        keyword("jump").padded().to(AsmOp::JUMP),
        keyword("call").padded().to(AsmOp::CALL),
        keyword("exit").padded().to(AsmOp::EXIT),
        keyword("trap").padded().to(AsmOp::TRAP),
    ))
    .padded()
    .padded_by(comment().repeated())
    .padded()
}

pub fn parse_asm<'a>() -> impl Parser<'a, &'a str, Object<'a>, extra::Err<Rich<'a, char>>> {
    let label = just(':').ignore_then(ident()).spanned().padded();

    label
        .then(parse_op().repeated().collect::<Vec<_>>())
        .repeated()
        .collect::<Vec<_>>()
        .validate(|e, _, emitter| {
            let mut out = IndexMap::new();

            for (label, asm) in e {
                if let Some(prev) = out.insert(label.inner, (label.span, asm)) {
                    emitter.emit(Rich::custom(prev.0, "previously defined here"));
                    emitter.emit(Rich::custom(label.span, "duplicate label"));
                }
            }

            Object(out.into_iter().map(|(k, (_, v))| (k.into(), v)).collect())
        })
        .padded_by(comment().repeated())
}

fn bytes_needed_for_number(n: u64) -> u64 {
    match n {
        0..=0xFF => 1,
        0x01_00..=0xFF_FF => 2,
        0x01_00_00..=0xFF_FF_FF => 3,
        0x01_00_00_00..=0xFF_FF_FF_FF => 4,
        0x01_00_00_00_00..=0xFF_FF_FF_FF_FF => 5,
        0x01_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF => 6,
        0x01_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF => 7,
        0x01_00_00_00_00_00_00_00..=0xFF_FF_FF_FF_FF_FF_FF_FF => 8,
    }
}

#[cfg(test)]
mod tests {
    use const_hex::ToHexExt;

    use super::*;
    use crate::Vm;

    fn init() {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::filter::EnvFilter::from_default_env())
            .try_init();
    }

    #[test]
    fn asm_op_push() {
        for (i, o) in [(1, AsmOp::PUSH1([1])), (0x01ff, AsmOp::PUSH2([0x01, 0xff]))] {
            assert_eq!(AsmOp::push(i), o);
        }
    }

    #[test]
    fn test_lit() {
        for (i, o) in [("0x01", [1]), (" 0xAA ", [0xaa])] {
            assert_eq!(lit::<1>().parse(i).unwrap(), o);
        }

        for (i, o) in [("0x010203ff", [1, 2, 3, 0xff]), (" 0x00000000 ", [0, 0, 0, 0])] {
            assert_eq!(lit::<4>().parse(i).unwrap(), o);
        }
    }

    #[test]
    fn test_ref() {
        for (i, o) in [
            ("@label", "label"),
            (" @label   ", "label"),
            ("@label  ", "label"),
            ("   @label", "label"),
        ] {
            assert_eq!(ref_().parse(i).unwrap(), o);
        }
    }

    #[test]
    fn test_parse_op() {
        for (i, o) in [
            ("push0", AsmOp::PUSH0),
            ("push1 0x00", AsmOp::PUSH1([0])),
            ("push2 0x0001", AsmOp::PUSH2([0, 1])),
            ("pushl @label", AsmOp::PUSHL("label".into())),
        ] {
            assert_eq!(parse_op().parse(i).unwrap(), o);
        }
    }

    #[test]
    fn test_parse_asm() {
        let asm = r"
:start
        ; init counter
        push0
        ; begin loop
:loop
        ; add 1
        push1 0x01
        add
        ; loop check
        push0
        dup
        push1 0x03
        sub
        ; jump to beginning of loop if the value on the top of the
        ; stack is non-zero (value - 3)
        pushl @loop
        jnz
        ; end loop
        ; init memory for value
        push1 0x01
        alloc
        ; update value in memory
        push0
        ; write ops take the value first then the pointer to write that value to in memory
        push0
        swap
        write1
        push0 ; ptr
        push1 0x01 ; len
        exit
        ";

        let object = parse_asm().parse(asm).unwrap();

        dbg!(&object);

        let asm = object.assemble();

        println!("{}", asm.encode_hex());

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        match res {
            Some(res) => {
                println!("res: {}", res.encode_hex());
            }
            None => {
                println!("res: <none>");
            }
        }
    }

    #[test]
    fn dedup() {
        init();

        let asm = r"
:start
        push1 0x01
        push1 0x01
        push1 0x01
";

        let object = parse_asm().parse(asm).unwrap();

        dbg!(&object);

        let asm = object.assemble();

        println!("{}", asm.encode_hex());

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        match res {
            Some(res) => {
                println!("res: {}", res.encode_hex());
            }
            None => {
                println!("res: <none>");
            }
        }
    }
}
