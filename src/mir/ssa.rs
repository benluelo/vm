use crate::mir::ast::{BuiltinOrDef, Ident, Val};

pub struct Assigment<'a> {
    vars: Vec<Ident<'a>>,
    expr: Expr<'a>,
}

pub enum Expr<'a> {
    Val(Val),
    Var(Ident<'a>),
    Call {
        f: BuiltinOrDef<'a>,
        args: Vec<ValOrVar<'a>>,
    },
}

pub enum ValOrVar<'a> {
    Val(Val),
    Var(Ident<'a>),
}

pub struct Block<'a>(Vec<Statement<'a>>);

pub enum Statement<'a> {
    Expr(Expr<'a>),
}
