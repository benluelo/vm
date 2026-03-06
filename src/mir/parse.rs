use std::fmt::{self, Write};

use chumsky::{
    prelude::*,
    text::{
        ascii::{self, keyword},
        digits, newline,
    },
};

#[cfg(test)]
mod tests;

macro_rules! Parser {
    ($a:lifetime, $ty:ty $(; [$($trait:ident),*])?) => {
        impl Parser<$a, &$a str, $ty, extra::Err<Rich<$a, char>>> + Clone $($(+ $trait)*)?
    };
}

pub struct Grammar<
    Comment,
    Val,
    Expr,
    Assignment,
    Statement,
    Loop,
    If,
    Label,
    Break,
    Continue,
    Block,
    Def,
> {
    pub comment: Comment,
    pub val: Val,
    pub expr: Expr,
    pub assignment: Assignment,
    pub statement: Statement,
    pub loop_: Loop,
    pub if_: If,
    pub label: Label,
    pub break_: Break,
    pub continue_: Continue,
    pub block: Block,
    pub def: Def,
}

#[allow(clippy::type_complexity)] // bitch
pub fn grammar<'a>() -> Grammar<
    Parser!['a, ()],
    Parser!['a, Val],
    Parser!['a, Expr<'a>],
    Parser!['a, Assignment<'a>],
    Parser!['a, Statement<'a>],
    Parser!['a, Loop<'a>],
    Parser!['a, If<'a>],
    Parser!['a, Label<'a>],
    Parser!['a, Break<'a>],
    Parser!['a, Continue<'a>],
    Parser!['a, Block<'a>],
    Parser!['a, Def<'a>],
> {
    fn ident<'a>() -> Parser!['a, Ident<'a>] {
        ascii::ident().spanned().padded().map(Ident)
    }

    fn comment<'a>() -> Parser!['a, ()] {
        just("#")
            .then(any().and_is(newline().not()).repeated())
            .padded()
            .ignored()
    }

    fn val<'a>() -> Parser!['a, Val] {
        just("0x")
            .ignore_then(
                digits(16)
                    .to_slice()
                    .map(|s| u64::from_str_radix(s, 16))
                    .unwrapped(),
            )
            .or(digits(10).to_slice().from_str().unwrapped())
            .spanned()
            .padded()
            .map(Val)
    }

    fn expr<'a>() -> Parser!['a, Expr<'a>] {
        recursive(|tree| {
            choice((
                just("...")
                    .or_not()
                    .then(
                        ident().then(
                            tree.padded()
                                .separated_by(just(','))
                                .collect::<Vec<_>>()
                                .delimited_by(just('('), just(')')),
                        ),
                    )
                    .map(|(spread, (f, args))| Expr::Call {
                        spread: spread.is_some(),
                        f,
                        args,
                    }),
                ident().map(Expr::Var),
                val().map(Expr::Val),
            ))
        })
        .padded()
        .labelled("arg expression")
    }

    fn assignment<'a>() -> Parser!['a, Assignment<'a>] {
        ident()
            .padded()
            .separated_by(just(','))
            .collect::<Vec<_>>()
            .then_ignore(just("<-").padded())
            .then(expr())
            .map(|(lhs, rhs)| Assignment(lhs, rhs))
            .labelled("assignment")
    }

    fn ident_list<'a>() -> Parser!['a, Vec<Ident<'a>>] {
        ident()
            .padded()
            .separated_by(just(','))
            .allow_trailing()
            .collect()
    }

    let mut statement = Recursive::declare();
    let mut loop_ = Recursive::declare();
    let mut if_ = Recursive::declare();
    let mut block = Recursive::declare();
    let mut def = Recursive::declare();

    loop_.define(
        keyword("loop")
            .padded()
            .ignore_then(label())
            .then(
                block
                    .clone()
                    .delimited_by(just('{').padded(), just('}').padded()),
            )
            .map(|(label, statements)| Loop(label, statements))
            .labelled("loop"),
    );

    if_.define(
        keyword("if")
            .padded()
            .ignore_then(expr().padded())
            .then(
                block
                    .clone()
                    .delimited_by(just('{').padded(), just('}').padded()),
            )
            .then(
                keyword("else")
                    .ignore_then(choice((
                        block
                            .clone()
                            .delimited_by(just('{').padded(), just('}').padded())
                            .map(|block| Else::Tail { block }),
                        if_.clone()
                            .spanned()
                            .map(|if_| Else::ElseIf { if_: Box::new(if_) }),
                    )))
                    .or_not(),
            )
            .map(|((cond, block), else_)| If { cond, block, else_ })
            .labelled("if"),
    );

    statement.define(
        choice((
            def.clone().map(Statement::Def),
            loop_.clone().map(Statement::Loop),
            break_().map(Statement::Break),
            continue_().map(Statement::Continue),
            if_.clone().map(Statement::If),
            assignment().map(Statement::Assignment),
            expr().map(Statement::Expr),
        ))
        .labelled("statement"),
    );

    block.define(
        statement
            .clone()
            // .then_ignore(newline())
            .padded()
            .repeated()
            .collect()
            .spanned()
            .map(Block)
            .padded(),
    );

    def.define(
        keyword("def")
            .padded()
            .ignore_then(ident().padded())
            .then(
                ident_list()
                    .delimited_by(just('(').padded(), just(')').padded())
                    .padded(),
            )
            .then_ignore(just("->").padded())
            .then(ident_list().padded())
            .then(
                block
                    .clone()
                    .delimited_by(just('{').padded(), just('}').padded()),
            )
            .map(|(((ident, args), ret), body)| Def {
                ident,
                args,
                rets: ret,
                body,
            })
            .labelled("def"),
    );

    fn label<'a>() -> Parser!['a, Label<'a>] {
        just(":")
            .ignore_then(ascii::ident().spanned().map(Label))
            .padded()
            .labelled("label")
    }

    fn break_<'a>() -> Parser!['a, Break<'a>] {
        keyword("break")
            .padded()
            .ignore_then(label())
            .map(Break)
            .labelled("break")
    }

    fn continue_<'a>() -> Parser!['a, Continue<'a>] {
        keyword("continue")
            .padded()
            .ignore_then(label())
            .map(Continue)
            .labelled("continue")
    }

    Grammar {
        comment: comment(),
        val: val(),
        expr: expr(),
        assignment: assignment(),
        statement,
        loop_,
        if_,
        label: label(),
        break_: break_(),
        continue_: continue_(),
        block,
        def,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Ident<'a>(pub Spanned<&'a str>);

impl<'a> PartialEq for Ident<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.inner == other.0.inner
    }
}

impl<'a> fmt::Display for Ident<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Val(pub Spanned<u64>);

impl Val {
    pub fn value(&self) -> u64 {
        self.0.inner
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0.inner, f)
    }
}

impl fmt::LowerHex for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0.inner, f)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Label<'a>(pub Spanned<&'a str>);

impl<'a> PartialEq for Label<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.inner == other.0.inner
    }
}

impl<'a> fmt::Display for Label<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Block<'a>(pub Spanned<Vec<Statement<'a>>>);

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Loop(Loop<'a>),
    Break(Break<'a>),
    Continue(Continue<'a>),
    If(If<'a>),
    Assignment(Assignment<'a>),
    Def(Def<'a>),
}

// TODO: Parse builtins directly?
#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Val(Val),
    Var(Ident<'a>),
    Call {
        spread: bool,
        f: Ident<'a>,
        args: Vec<Expr<'a>>,
    },
}

impl Expr<'_> {
    pub(crate) fn span(&self) -> SimpleSpan {
        match self {
            Expr::Val(val) => val.0.span,
            Expr::Var(var) => var.0.span,
            Expr::Call {
                spread: _,
                f,
                args: _,
            } => f.0.span,
        }
    }
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Val(val) => f.write_fmt(format_args!("{}", val)),
            Expr::Var(var) => f.write_fmt(format_args!("{var}")),
            Expr::Call {
                spread,
                f: call,
                args,
            } => {
                if *spread {
                    f.write_str("...")?;
                }
                f.write_fmt(format_args!("{call}"))?;
                f.write_char('(')?;
                for (i, expr) in args.iter().enumerate() {
                    f.write_fmt(format_args!("{expr}"))?;
                    if i < args.len() - 1 {
                        f.write_str(", ")?;
                    }
                }
                f.write_char(')')
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Break<'a>(pub Label<'a>);

#[derive(Debug, Clone)]
pub struct Continue<'a>(pub Label<'a>);

#[derive(Debug, Clone)]
pub struct Loop<'a>(pub Label<'a>, pub Block<'a>);

#[derive(Debug, Clone)]
pub struct If<'a> {
    pub cond: Expr<'a>,
    pub block: Block<'a>,
    pub else_: Option<Else<'a>>,
}

#[derive(Debug, Clone)]
pub enum Else<'a> {
    ElseIf { if_: Box<Spanned<If<'a>>> },
    Tail { block: Block<'a> },
}

#[derive(Debug, Clone)]
pub struct Assignment<'a>(pub Vec<Ident<'a>>, pub Expr<'a>);

#[derive(Debug, Clone)]
pub struct Def<'a> {
    pub ident: Ident<'a>,
    pub args: Vec<Ident<'a>>,
    pub rets: Vec<Ident<'a>>,
    pub body: Block<'a>,
}

fn print_ast(ast: &Block<'_>) -> String {
    fn go(depth: usize, out: &mut String, ast: &Block<'_>) {
        for s in &ast.0.inner {
            match s {
                Statement::Expr(expr) => {
                    out.push_str(&"  ".repeat(depth));
                    go_expr(out, expr);
                }
                Statement::Loop(Loop(label, block)) => {
                    out.push_str(&"  ".repeat(depth));
                    out.push_str("loop :");
                    write!(out, "{}", label).unwrap();
                    out.push_str(" {\n");
                    go(depth + 1, out, block);
                    out.write_str(&"  ".repeat(depth)).unwrap();
                    out.push('}');
                }
                Statement::Break(label) => {
                    out.push_str(&"  ".repeat(depth));
                    out.push_str("break :");
                    write!(out, "{}", label.0).unwrap();
                }
                Statement::Continue(label) => {
                    out.push_str(&"  ".repeat(depth));
                    out.push_str("continue :");
                    write!(out, "{}", label.0).unwrap();
                }
                Statement::If(if_) => {
                    fn go_if(depth: usize, out: &mut String, If { cond, block, else_ }: &If) {
                        out.push_str("if ");
                        go_expr(out, cond);
                        out.push_str(" {\n");
                        go(depth + 1, out, block);
                        out.write_str(&"  ".repeat(depth)).unwrap();
                        out.push('}');
                        match else_ {
                            Some(Else::ElseIf { if_ }) => {
                                go_if(depth, out, if_);
                            }
                            Some(Else::Tail { block }) => {
                                out.push_str(" {\n");
                                go(depth + 1, out, block);
                                out.write_str(&"  ".repeat(depth)).unwrap();
                                out.push('}');
                            }
                            None => {}
                        }
                    }

                    out.push_str(&"  ".repeat(depth));

                    go_if(depth, out, if_);
                }
                Statement::Assignment(Assignment(vars, expr)) => {
                    out.push_str(&"  ".repeat(depth));
                    for (i, var) in vars.iter().enumerate() {
                        write!(out, "{var}").unwrap();
                        if i == vars.len() {
                            out.push_str(", ");
                        }
                    }
                    out.push_str(" <- ");
                    go_expr(out, expr);
                }
                Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body,
                }) => {
                    out.push_str(&"  ".repeat(depth));
                    out.push_str("def ");
                    write!(out, "{ident}").unwrap();
                    out.push('(');
                    for (i, arg) in args.iter().enumerate() {
                        write!(out, "{arg}").unwrap();
                        if i == args.len() {
                            out.push_str(", ");
                        }
                    }
                    out.push(')');
                    out.push_str(" -> ");
                    for (i, ret) in rets.iter().enumerate() {
                        write!(out, "{ret}").unwrap();
                        if i == rets.len() {
                            out.push_str(", ");
                        }
                    }
                    out.push_str(" {\n");
                    go(depth + 1, out, body);
                    out.write_str(&"  ".repeat(depth)).unwrap();
                    out.push('}');
                }
            }
            out.push('\n');
        }
    }

    fn go_expr(out: &mut String, expr: &Expr<'_>) {
        match expr {
            Expr::Val(val) => out.write_fmt(format_args!("{val}")).unwrap(),
            Expr::Var(var) => write!(out, "{var}").unwrap(),
            Expr::Call {
                spread,
                f,
                args: exprs,
            } => {
                if *spread {
                    write!(out, "...").unwrap();
                }
                write!(out, "{f}").unwrap();
                out.push('(');
                for (i, expr) in exprs.iter().enumerate() {
                    go_expr(out, expr);
                    if i < exprs.len() - 1 {
                        out.push_str(", ");
                    }
                }
                out.push(')');
            }
        }
    }

    let mut out = String::new();

    go(0, &mut out, ast);

    out
}
