use std::{collections::BTreeMap, fmt};

macro_rules! id {
    ($Ty:ident, $prefix:literal) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $Ty(usize);

        impl crate::mir::ssa::id_map::Id for $Ty {
            fn as_usize(self) -> usize {
                self.0
            }

            fn from_usize(id: usize) -> Self {
                Self(id)
            }
        }

        impl fmt::Display for $Ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($prefix, "{}"), self.0)
            }
        }
    };
}

#[derive(Debug)]
pub struct Builder {
    blocks: BTreeMap<BlockId, Block>,
    defs: BTreeMap<DefId, Def>,
}

impl Builder {
    pub(crate) fn new() -> Self {
        Self {
            blocks: BTreeMap::new(),
            defs: BTreeMap::new(),
        }
    }
}

id!(BlockId, "bb");
id!(VarId, "t");
id!(DefId, "f");

#[derive(Debug, Clone)]
pub struct Block {
    args: Vec<VarId>,
    instructions: Vec<Statement>,
    terminal: Terminal,
}

#[derive(Debug, Clone)]
pub enum Statement {
    /// Call to a builtin
    Builtin(BuiltinCall),
    /// Assignment of an expression to a variable
    Assignment { var: VarId, expr: Expr },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Builtin(builtin_call) => write!(f, "{builtin_call}"),
            Statement::Assignment { var, expr } => write!(f, "{var} <- {expr}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    /// Constant value
    Const(u64),
    /// Block-local variable
    Var(VarId),
    /// Call to a builtin
    Builtin(BuiltinCall),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{c}"),
            Expr::Var(v) => write!(f, "{v}"),
            Expr::Builtin(builtin_call) => write!(f, "{builtin_call}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuiltinCall {
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Exp(Operand, Operand),
    Mod(Operand, Operand),
    Eq(Operand, Operand),
    Lt(Operand, Operand),
    Gt(Operand, Operand),
    Shl(Operand, Operand),
    Shr(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),
    And(Operand, Operand),
    Not(Operand),
    Neg(Operand),
    Dread1(Operand),
    Dread2(Operand),
    Dread3(Operand),
    Dread4(Operand),
    Dread5(Operand),
    Dread6(Operand),
    Dread7(Operand),
    Dread8(Operand),
    Dlen(),
    Read1(Operand),
    Read2(Operand),
    Read3(Operand),
    Read4(Operand),
    Read5(Operand),
    Read6(Operand),
    Read7(Operand),
    Read8(Operand),
    Alloc(Operand),
    Write1(Operand, Operand),
    Write2(Operand, Operand),
    Write3(Operand, Operand),
    Write4(Operand, Operand),
    Write5(Operand, Operand),
    Write6(Operand, Operand),
    Write7(Operand, Operand),
    Write8(Operand, Operand),
    Dcopy(Operand, Operand, Operand),
    Exit(Operand, Operand),
    Trap(Operand),
}

impl fmt::Display for BuiltinCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinCall::Add(a, b) => write!(f, "add({a}, {b})"),
            BuiltinCall::Sub(a, b) => write!(f, "sub({a}, {b})"),
            BuiltinCall::Mul(a, b) => write!(f, "mul({a}, {b})"),
            BuiltinCall::Div(a, b) => write!(f, "div({a}, {b})"),
            BuiltinCall::Exp(a, b) => write!(f, "exp({a}, {b})"),
            BuiltinCall::Mod(a, b) => write!(f, "mod({a}, {b})"),
            BuiltinCall::Eq(a, b) => write!(f, "eq({a}, {b})"),
            BuiltinCall::Lt(a, b) => write!(f, "lt({a}, {b})"),
            BuiltinCall::Gt(a, b) => write!(f, "gt({a}, {b})"),
            BuiltinCall::Shl(a, b) => write!(f, "shl({a}, {b})"),
            BuiltinCall::Shr(a, b) => write!(f, "shr({a}, {b})"),
            BuiltinCall::Or(a, b) => write!(f, "or({a}, {b})"),
            BuiltinCall::Xor(a, b) => write!(f, "xor({a}, {b})"),
            BuiltinCall::And(a, b) => write!(f, "and({a}, {b})"),
            BuiltinCall::Not(a) => write!(f, "not({a})"),
            BuiltinCall::Neg(a) => write!(f, "neg({a})"),
            BuiltinCall::Dread1(a) => write!(f, "dread1({a})"),
            BuiltinCall::Dread2(a) => write!(f, "dread2({a})"),
            BuiltinCall::Dread3(a) => write!(f, "dread3({a})"),
            BuiltinCall::Dread4(a) => write!(f, "dread4({a})"),
            BuiltinCall::Dread5(a) => write!(f, "dread5({a})"),
            BuiltinCall::Dread6(a) => write!(f, "dread6({a})"),
            BuiltinCall::Dread7(a) => write!(f, "dread7({a})"),
            BuiltinCall::Dread8(a) => write!(f, "dread8({a})"),
            BuiltinCall::Dlen() => write!(f, "dlen()"),
            BuiltinCall::Read1(a) => write!(f, "read1({a})"),
            BuiltinCall::Read2(a) => write!(f, "read2({a})"),
            BuiltinCall::Read3(a) => write!(f, "read3({a})"),
            BuiltinCall::Read4(a) => write!(f, "read4({a})"),
            BuiltinCall::Read5(a) => write!(f, "read5({a})"),
            BuiltinCall::Read6(a) => write!(f, "read6({a})"),
            BuiltinCall::Read7(a) => write!(f, "read7({a})"),
            BuiltinCall::Read8(a) => write!(f, "read8({a})"),
            BuiltinCall::Alloc(a) => write!(f, "alloc({a})"),
            BuiltinCall::Write1(a, b) => write!(f, "write1({a}, {b})"),
            BuiltinCall::Write2(a, b) => write!(f, "write2({a}, {b})"),
            BuiltinCall::Write3(a, b) => write!(f, "write3({a}, {b})"),
            BuiltinCall::Write4(a, b) => write!(f, "write4({a}, {b})"),
            BuiltinCall::Write5(a, b) => write!(f, "write5({a}, {b})"),
            BuiltinCall::Write6(a, b) => write!(f, "write6({a}, {b})"),
            BuiltinCall::Write7(a, b) => write!(f, "write7({a}, {b})"),
            BuiltinCall::Write8(a, b) => write!(f, "write8({a}, {b})"),
            BuiltinCall::Dcopy(a, b, o2) => write!(f, "dcopy({a}, {b}, {o2})"),
            BuiltinCall::Exit(a, b) => write!(f, "exit({a}, {b})"),
            BuiltinCall::Trap(a) => write!(f, "trap({a})"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Const(u64),
    Var(VarId),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Const(c) => write!(f, "{c}"),
            Operand::Var(v) => write!(f, "{v}"),
        }
    }
}

impl Operand {
    // fn from_ast_expr(expr: Expr<'_>) -> Self {
    //     match expr {
    //         Expr::Val(val) => Operand::Const(val.value()),
    //         Expr::Var(ident) => Operand::Var(ident.to_string()),
    //         Expr::Call { spread, f, args } => {
    //             panic!("not normalized: {}", Expr::Call { spread, f, args })
    //         }
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum Terminal {
    /// Conditional jump to the provided label
    Jump {
        cond: Expr,
        then: Goto,
        else_: Goto,
    },
    /// Unconditional jump to the provided goto label
    Goto(Goto),
    // Trap (Operand),
    // Exit (Operand, Operand),
    End,
}

impl fmt::Display for Terminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminal::Jump { cond, then, else_ } => write!(f, "if {cond} then {then} else {else_}"),
            Terminal::Goto(goto) => write!(f, "goto {goto}"),
            Terminal::End => write!(f, "end"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Goto {
    label: BlockId,
    args: Vec<Operand>,
}

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label)?;
        if !self.args.is_empty() {
            write!(f, "(")?;
            for (idx, arg) in self.args.iter().enumerate() {
                write!(f, "{arg}")?;
                if idx < self.args.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Def {
    params: Vec<()>,
    rets: Vec<()>,
    blocks: BTreeMap<BlockId, Block>,
}
