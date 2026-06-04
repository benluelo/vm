use chumsky::span::{SimpleSpan, Spanned};
use tracing::info;

use super::parse::{BuiltinOrDef, Expr, Statement};
use crate::{
    mir::{
        CheckCtx,
        parse::{Assignment, Block, Def, Else, Ident, If, Loop, Val},
    },
    op,
};

pub trait Pass {
    fn run<'a>(&mut self, check_ctx: &CheckCtx<'a>, block: Block<'a>) -> Block<'a>;
}

pub struct ConstEval {}

impl ConstEval {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Pass for ConstEval {
    fn run<'a>(&mut self, check_ctx: &CheckCtx<'a>, block: Block<'a>) -> Block<'a> {
        let mut new_block = vec![];
        let span = block.span();

        for statement in block {
            let new_statement = match statement {
                Statement::Expr(expr) => Statement::Expr(const_eval(expr)),
                Statement::Loop(Loop(label, block)) => {
                    Statement::Loop(Loop(label, self.run(check_ctx, block)))
                }
                Statement::If(If { cond, block, else_ }) => Statement::If(If {
                    cond: const_eval(cond),
                    block: self.run(check_ctx, block),
                    // TODO: Run on else blocks
                    else_,
                }),
                Statement::Assignment(Assignment(lhs, rhs)) => {
                    Statement::Assignment(Assignment(lhs, const_eval(rhs)))
                }
                Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body,
                }) => Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body: self.run(check_ctx, body),
                }),
                _ => statement,
            };

            new_block.push(new_statement);
        }

        Block::new(new_block, span)
    }
}

fn const_eval<'a>(expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Val(val) => Expr::Val(val),
        Expr::Var(var) => Expr::Var(var),
        Expr::Call { spread, f, args } => {
            use BuiltinOrDef::*;

            // macro_rules! binop {
            //     ($ctor:ident, ) => {

            //     };
            // }

            let len = args.len();

            let binop = |ctor: BuiltinOrDef<'a>, f_: fn(u64, u64) -> u64| -> Expr<'a> {
                match (const_eval(args[0].clone()), const_eval(args[1].clone())) {
                    (Expr::Val(l), Expr::Val(r)) => Expr::Val(Val(Spanned {
                        inner: f_(l.0.inner, r.0.inner),
                        span: f.span,
                    })),
                    (l, r) => Expr::Call {
                        spread,
                        f: Spanned {
                            inner: ctor,
                            span: f.span,
                        },
                        args: vec![l, r],
                    },
                }
            };

            match (f.inner, len) {
                (Add, 2) => binop(Add, op::add),
                (Sub, 2) => binop(Sub, op::sub),
                (Mul, 2) => binop(Mul, op::mul),
                // Div => todo!(),
                // Exp => todo!(),
                (Mod, 2) => binop(Mod, op::r#mod),
                (Eq, 2) => binop(Eq, op::eq),
                (Lt, 2) => binop(Lt, op::lt),
                (Gt, 2) => binop(Gt, op::gt),
                // Shl => todo!(),
                // Shr => todo!(),
                (Or, 2) => binop(Or, op::or),
                (Xor, 2) => binop(Xor, op::xor),
                (And, 2) => binop(And, op::and),
                // Not => todo!(),
                // Neg => todo!(),
                (f_, _) => Expr::Call {
                    spread,
                    f: Spanned {
                        inner: f_,
                        span: f.span,
                    },
                    args: args.into_iter().map(const_eval).collect(),
                },
            }
        }
    }
}

pub struct DefInline {}

impl DefInline {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    fn run_on_if_statement<'a>(&mut self, check_ctx: &CheckCtx<'a>, if_: If<'a>) -> If<'a> {
        If {
            cond: def_inline(check_ctx, if_.cond),
            block: self.run(check_ctx, if_.block),
            else_: match if_.else_ {
                Some(else_) => match else_ {
                    Else::ElseIf { if_ } => Some(Else::ElseIf {
                        if_: Box::new(Spanned {
                            inner: self.run_on_if_statement(check_ctx, if_.inner),
                            span: if_.span,
                        }),
                    }),
                    Else::Tail { block } => Some(Else::Tail {
                        block: self.run(check_ctx, block),
                    }),
                },
                None => None,
            },
        }
    }
}

impl Pass for DefInline {
    fn run<'a>(&mut self, check_ctx: &CheckCtx<'a>, block: Block<'a>) -> Block<'a> {
        let mut new_block = vec![];
        let span = block.span();

        for statement in block {
            let new_statement = match statement {
                Statement::Expr(expr) => Statement::Expr(def_inline(check_ctx, expr)),
                Statement::Loop(Loop(label, block)) => {
                    Statement::Loop(Loop(label, self.run(check_ctx, block)))
                }
                Statement::If(if_) => Statement::If(self.run_on_if_statement(check_ctx, if_)),
                Statement::Assignment(Assignment(lhs, rhs)) => {
                    Statement::Assignment(Assignment(lhs, def_inline(check_ctx, rhs)))
                }
                Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body,
                }) => Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body: self.run(check_ctx, body),
                }),
                _ => statement,
            };

            new_block.push(new_statement);
        }

        Block::new(new_block, span)
    }
}

fn def_inline<'a>(check_ctx: &CheckCtx<'a>, expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Val(val) => Expr::Val(val),
        Expr::Var(var) => Expr::Var(var),
        Expr::Call { spread, f, args } => {
            let inline_args = |f, args: Vec<_>| Expr::Call {
                spread,
                f,
                args: args
                    .into_iter()
                    .map(|arg| def_inline(check_ctx, arg))
                    .collect(),
            };

            if let BuiltinOrDef::Def(f_) = *f {
                let def = check_ctx.get_def(&f_).unwrap();
                if def.rets.len() == 1
                    && def.body.len() == 1
                    && let Statement::Assignment(assignment) = def.body.iter().next().unwrap()
                    && assignment.0.len() == 1
                    && assignment.0[0].0.inner == def.rets[0].0.inner
                {
                    info!("inlining");
                    let mut a = assignment.1.clone();

                    inline_def_args(&mut a, &args, &def.args);

                    a
                } else if def.body.len() == 1
                    && let Statement::Expr(expr) = def.body.iter().next().unwrap()
                {
                    info!("inlining");
                    let mut expr = expr.clone();

                    inline_def_args(&mut expr, &args, &def.args);

                    expr
                } else {
                    inline_args(f, args)
                }
            } else {
                inline_args(f, args)
            }
        }
    }
}

fn inline_def_args<'a>(expr: &mut Expr<'a>, params: &[Expr<'a>], args: &[Ident<'a>]) {
    match expr {
        Expr::Val(_) => {}
        Expr::Var(ident) => {
            if let Some(idx) = args.iter().position(|n| n == ident) {
                *expr = params[idx].clone();
            }
        }
        Expr::Call {
            spread: _,
            f: _,
            args: call_args,
        } => {
            for ca in call_args {
                inline_def_args(ca, params, args)
            }
        }
    }
}

pub struct DeadCodeRemoval {}

impl DeadCodeRemoval {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

// impl Pass for DeadCodeRemoval {
//     fn run<'a>(&mut self, check_ctx: &CheckCtx<'a>, block: Block<'a>) ->
// Block<'a> {         let mut new_block = vec![];
//         let span = block.span();

//         for statement in block {
//             let new_statement = match statement {
//                 // TODO: Remove pure exprs
//                 // Statement::Expr(expr) =>
// Statement::Expr(const_eval(expr)),
// Statement::Loop(Loop(label, block)) => {
// Statement::Loop(Loop(label, self.run(check_ctx, block)))                 }
//                 Statement::If(If { cond, block, else_ }) => Statement::If(If
// {                     cond: const_eval(cond),
//                     block: self.run(check_ctx, block),
//                     // TODO: Run on else blocks
//                     else_,
//                 }),
//                 Statement::Assignment(Assignment(lhs, rhs)) => {
//                     Statement::Assignment(Assignment(lhs, const_eval(rhs)))
//                 }
//                 Statement::Def(Def {
//                     ident,
//                     args,
//                     rets,
//                     body,
//                 }) => Statement::Def(Def {
//                     ident,
//                     args,
//                     rets,
//                     body: self.run(check_ctx, body),
//                 }),
//                 _ => statement,
//             };

//             new_block.push(new_statement);
//         }

//         Block::new(new_block, span)
//     }
// }

// fn count_uses_of_def(def: Ident, block: &Block) -> u32 {
//     block
//         .iter()
//         .map(|stmt| {
//             fn go_expr(def: Ident, expr: &Expr) -> u32 {
//                 match expr {
//                     Expr::Val(val) => 0,
//                     Expr::Var(ident) => 0,
//                     Expr::Call { spread, f, args } => {
//                         (if let BuiltinOrDef::Def(f) = &f.inner
//                             && f == &def
//                         {
//                             1
//                         } else {
//                             0
//                         }) + args.iter().map(|arg| go_expr(def,
// arg)).sum::<u32>()                     }
//                 }
//             }

//             match stmt {
//                 Statement::Expr(expr) => go_expr(def, expr),
//                 Statement::Loop(loop_) => count_uses_of_def(def, &loop_.1),
//                 Statement::Break(_) => 0,
//                 Statement::Continue(_) => 0,
//                 Statement::If(if_) => {
//                     fn go_if(def: Ident, if_: &If) -> u32 {
//                         go_expr(def, &if_.cond)
//                             + count_uses_of_def(def, &if_.block)
//                             + if_.else_.as_ref().map_or(0, |else_| match
//                               else_ { Else::ElseIf { if_ } => go_if(def,
//                               if_), Else::Tail { block } =>
//                               count_uses_of_def(def, block),
//                             })
//                     }

//                     go_if(def, if_)
//                 }
//                 Statement::Assignment(assignment) => go_expr(def,
// &assignment.1),                 Statement::Def(def_) =>
// count_uses_of_def(def, &def_.body),             }
//         })
//         .sum()
// }

// fn dead_code_removal<'a>(expr: Expr<'a>) -> Expr<'a> {
//     match expr {
//         Expr::Val(val) => Expr::Val(val),
//         Expr::Var(var) => Expr::Var(var),
//         Expr::Call { spread, f, args } => {
//             use BuiltinOrDef::*;

//             // macro_rules! binop {
//             //     ($ctor:ident, ) => {

//             //     };
//             // }

//             let len = args.len();

//             let binop = |ctor: BuiltinOrDef<'a>, f_: fn(u64, u64) -> u64| ->
// Expr<'a> {                 match (const_eval(args[0].clone()),
// const_eval(args[1].clone())) {                     (Expr::Val(l),
// Expr::Val(r)) => Expr::Val(Val(Spanned {                         inner:
// f_(l.0.inner, r.0.inner),                         span: f.span,
//                     })),
//                     (l, r) => Expr::Call {
//                         spread,
//                         f: Spanned {
//                             inner: ctor,
//                             span: f.span,
//                         },
//                         args: vec![l, r],
//                     },
//                 }
//             };

//             match (f.inner, len) {
//                 (Add, 2) => binop(Add, op::add),
//                 (Sub, 2) => binop(Sub, op::sub),
//                 (Mul, 2) => binop(Mul, op::mul),
//                 // Div => todo!(),
//                 // Exp => todo!(),
//                 (Mod, 2) => binop(Mod, op::r#mod),
//                 (Eq, 2) => binop(Eq, op::eq),
//                 (Lt, 2) => binop(Lt, op::lt),
//                 (Gt, 2) => binop(Gt, op::gt),
//                 // Shl => todo!(),
//                 // Shr => todo!(),
//                 (Or, 2) => binop(Or, op::or),
//                 (Xor, 2) => binop(Xor, op::xor),
//                 (And, 2) => binop(And, op::and),
//                 // Not => todo!(),
//                 // Neg => todo!(),
//                 (f_, _) => Expr::Call {
//                     spread,
//                     f: Spanned {
//                         inner: f_,
//                         span: f.span,
//                     },
//                     args: args.into_iter().map(const_eval).collect(),
//                 },
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use chumsky::Parser;
    use petgraph::{dot::Dot, graph::DiGraph, prelude::NodeIndex};

    use super::*;
    use crate::mir::parse::grammar;

    #[test]
    fn cfg() {
        // enum BasicBlockComponent<'a> {
        //     Expr(Expr<'a>),
        //     Assignment(Assignment<'a>),
        // }

        #[derive(Debug)]
        enum BasicBlock<'a> {
            Root,
            Expr(Expr<'a>),
            Assignment(Assignment<'a>),
        }

        impl Display for BasicBlock<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    BasicBlock::Root => f.write_str("root"),
                    BasicBlock::Expr(expr) => f.write_fmt(format_args!("{expr}")),
                    BasicBlock::Assignment(assignment) => f.write_fmt(format_args!("{assignment}")),
                }
            }
        }

        #[derive(Debug)]
        enum Edge {
            None,
            Break,
            Continue,
            IfTrue,
            IfFalse,
        }

        impl Display for Edge {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Edge::None => f.write_str(""),
                    Edge::Break => f.write_str("break"),
                    Edge::Continue => f.write_str("continue"),
                    Edge::IfTrue => f.write_str("ifTrue"),
                    Edge::IfFalse => f.write_str("ifFalse"),
                }
            }
        }

        let input = r"
            x <- 1
            y <- 2
            z <- 0
            if lt(x, y) {
                z <- 1
            } else {
                z <- 2
            }
            alloc(z)
        ";

        let ast = grammar().block.parse(input).unwrap();

        let mut cfg = DiGraph::<BasicBlock, Edge>::new();
        let mut last_node = cfg.add_node(BasicBlock::Root);

        fn go_block<'a>(
            cfg: &mut DiGraph<BasicBlock<'a>, Edge>,
            last_node: &mut NodeIndex,
            mut edge: Edge,
            block: Block<'a>,
        ) {
            for stmt in block {
                match stmt {
                    Statement::Expr(expr) => {
                        let this_node = cfg.add_node(BasicBlock::Expr(expr));
                        cfg.add_edge(*last_node, this_node, edge);
                        edge = Edge::None;
                        *last_node = this_node;
                    }
                    Statement::Loop(_) => todo!(),
                    Statement::Break(_) => todo!(),
                    Statement::Continue(_) => todo!(),
                    Statement::If(if_) => {
                        fn go_if<'a>(
                            cfg: &mut DiGraph<BasicBlock<'a>, Edge>,
                            last_node: &mut NodeIndex,
                            if_: If<'a>,
                        ) {
                            let cond_node = cfg.add_node(BasicBlock::Expr(if_.cond));
                            cfg.add_edge(*last_node, cond_node, Edge::None);
                            *last_node = cond_node;

                            go_block(cfg, last_node, Edge::IfTrue, if_.block);

                            match if_.else_ {
                                Some(else_) => match else_ {
                                    Else::ElseIf { if_ } => todo!(),
                                    Else::Tail { block } => {
                                        *last_node = cond_node;
                                        go_block(cfg, last_node, Edge::IfFalse, block);
                                    }
                                },
                                None => todo!(),
                            }
                        }

                        go_if(cfg, last_node, if_);
                    }
                    Statement::Assignment(assignment) => {
                        let this_node = cfg.add_node(BasicBlock::Assignment(assignment));
                        cfg.add_edge(*last_node, this_node, Edge::None);
                        *last_node = this_node;
                    }
                    Statement::Def(def) => todo!(),
                }
            }
        }

        go_block(&mut cfg, &mut last_node, Edge::None, ast);

        println!("{input}");

        println!("{}", Dot::new(&cfg));

        // dbg!(cfg);
    }
}
