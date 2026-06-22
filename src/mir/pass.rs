use chumsky::{container::Container, span::Spanned};
use tracing::info;

use crate::{
    mir::{
        CheckCtx,
        ast::{
            Assignment, Block, Builtin, BuiltinOrDef, Def, Else, Expr, Ident, If, Loop, Statement,
            Val,
        },
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
                Statement::Loop(Loop { label, block }) => Statement::Loop(Loop {
                    label,
                    block: self.run(check_ctx, block),
                }),
                Statement::If(If { cond, block, else_ }) => Statement::If(If {
                    cond: const_eval(cond),
                    block: self.run(check_ctx, block),
                    // TODO: Run on else blocks
                    else_,
                }),
                Statement::Assignment(Assignment { vars, expr }) => {
                    Statement::Assignment(Assignment {
                        vars,
                        expr: const_eval(expr),
                    })
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

impl Normalize {
    fn next_var(&mut self) -> Ident<'static> {
        let id = self.counter;
        self.counter += 1;
        Ident::new(format!("t{id}"))
    }

    fn normalize_expr<'a>(&mut self, expr: Expr<'a>) -> (Vec<Statement<'a>>, Expr<'a>) {
        match expr {
            Expr::Val(val) => (vec![], Expr::Val(val)),
            Expr::Var(var) => (vec![], Expr::Var(var)),
            Expr::Call { spread, f, args } => {
                if args.iter().any(|a| matches!(a, Expr::Call { .. })) {
                    let mut statements = vec![];
                    // let mut last_var = None;
                    let args_len = args.len();
                    let mut new_args = vec![];
                    for (idx, arg) in args.into_iter().enumerate() {
                        match arg {
                            expr @ (Expr::Var(_) | Expr::Val(_)) => new_args.push(expr),
                            arg @ Expr::Call { .. } => {
                                let (s, expr) = self.normalize_expr(arg);
                                statements.extend(s);
                                let next_var = self.next_var();
                                new_args.push(Expr::Var(next_var.clone()));
                                statements.push(Statement::Assignment(Assignment {
                                    vars: vec![next_var],
                                    expr,
                                }));
                            }
                        }
                    }
                    (
                        statements,
                        Expr::Call {
                            spread,
                            f,
                            args: new_args,
                        },
                    )
                } else {
                    (vec![], Expr::Call { spread, f, args })
                }
            }
        }
    }

    fn normalize_if<'a>(
        &mut self,
        check_ctx: &CheckCtx<'a>,
        If { cond, block, else_ }: If<'a>,
    ) -> (Vec<Statement<'a>>, If<'a>) {
        let mut new_block = vec![];
        let (statements, cond_expr) = self.normalize_expr(cond);
        new_block.extend(statements);
        let if_ = If {
            cond: cond_expr,
            block: self.run(check_ctx, block),
            // TODO: Run on else blocks
            else_: match else_ {
                Some(Else::ElseIf { if_ }) => {
                    let (statements, new_if) = self.normalize_if(check_ctx, if_.inner);
                    new_block.extend(statements);
                    Some(Else::ElseIf {
                        if_: Box::new(Spanned {
                            inner: new_if,
                            span: if_.span,
                        }),
                    })
                }
                Some(Else::Tail { block }) => Some(Else::Tail {
                    block: self.run(check_ctx, block),
                }),
                None => None,
            },
        };

        (new_block, if_)
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
                Statement::Loop(Loop { label, block }) => Statement::Loop(Loop {
                    label,
                    block: self.run(check_ctx, block),
                }),
                Statement::If(if_) => Statement::If(self.run_on_if_statement(check_ctx, if_)),
                Statement::Assignment(Assignment { vars, expr }) => {
                    Statement::Assignment(Assignment {
                        vars,
                        expr: def_inline(check_ctx, expr),
                    })
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

            if let BuiltinOrDef::Def(f_) = &*f {
                let def = check_ctx.get_def(&f_).unwrap();
                if def.rets.len() == 1
                    && def.body.len() == 1
                    && let Statement::Assignment(assignment) = def.body.iter().next().unwrap()
                    && assignment.vars.len() == 1
                    && assignment.vars[0] == def.rets[0]
                {
                    info!("inlining");
                    let mut a = assignment.expr.clone();

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

pub struct Normalize {
    counter: u32,
}

impl Normalize {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Pass for Normalize {
    fn run<'a>(&mut self, check_ctx: &CheckCtx<'a>, block: Block<'a>) -> Block<'a> {
        let mut new_block = vec![];
        let span = block.span();

        for statement in block {
            match statement {
                Statement::Expr(expr) => {
                    let (statements, expr) = self.normalize_expr(expr);
                    new_block.extend(statements);
                    new_block.push(Statement::Expr(expr));
                }
                Statement::Loop(Loop { label, block }) => new_block.push(Statement::Loop(Loop {
                    label,
                    block: self.run(check_ctx, block),
                })),
                Statement::If(if_) => {
                    let (statements, if_) = self.normalize_if(check_ctx, if_);
                    new_block.extend(statements);
                    new_block.push(Statement::If(if_));
                }
                Statement::Assignment(Assignment { vars, expr }) => {
                    let (statements, expr) = self.normalize_expr(expr);
                    new_block.extend(statements);
                    new_block.push(Statement::Assignment(Assignment {
                        vars,
                        expr: const_eval(expr),
                    }));
                }
                Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body,
                }) => new_block.push(Statement::Def(Def {
                    ident,
                    args,
                    rets,
                    body: self.run(check_ctx, body),
                })),
                _ => new_block.push(statement),
            };
        }

        Block::new(new_block, span)
    }
}

fn const_eval<'a>(expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Val(val) => Expr::Val(val),
        Expr::Var(var) => Expr::Var(var),
        Expr::Call { spread, f, args } => {
            use Builtin::*;

            let len = args.len();

            let binop = |ctor: Builtin, f_: fn(u64, u64) -> u64| -> Expr<'a> {
                match (const_eval(args[0].clone()), const_eval(args[1].clone())) {
                    (Expr::Val(l), Expr::Val(r)) => Expr::Val(Val::new_spanned(Spanned {
                        inner: f_(l.value(), r.value()),
                        span: f.span,
                    })),
                    (l, r) => Expr::Call {
                        spread,
                        f: Spanned {
                            inner: ctor.into(),
                            span: f.span,
                        },
                        args: vec![l, r],
                    },
                }
            };

            match (f.inner, len) {
                (BuiltinOrDef::Builtin(Add), 2) => binop(Add, op::add),
                (BuiltinOrDef::Builtin(Sub), 2) => binop(Sub, |l, r| op::sub(r, l)),
                (BuiltinOrDef::Builtin(Mul), 2) => binop(Mul, op::mul),
                // Div => todo!(),
                // Exp => todo!(),
                (BuiltinOrDef::Builtin(Mod), 2) => binop(Mod, |a, b| op::r#mod(a, b).unwrap()),
                (BuiltinOrDef::Builtin(Eq), 2) => binop(Eq, op::eq),
                (BuiltinOrDef::Builtin(Lt), 2) => binop(Lt, op::lt),
                (BuiltinOrDef::Builtin(Gt), 2) => binop(Gt, op::gt),
                // Shl => todo!(),
                // Shr => todo!(),
                (BuiltinOrDef::Builtin(Or), 2) => binop(Or, op::or),
                (BuiltinOrDef::Builtin(Xor), 2) => binop(Xor, op::xor),
                (BuiltinOrDef::Builtin(And), 2) => binop(And, op::and),
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

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use chumsky::Parser;
    use petgraph::{dot::Dot, graph::DiGraph, prelude::NodeIndex};

    use super::*;
    use crate::mir::parse::{grammar, print_ast};

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

    #[test]
    fn normalize() {
        let raw = r"
# Function to load a 64-bit value using the little-endian (LE) convention.
# On a LE platform, this could be greatly simplified using a cast.
def load64(at) -> u {
  # i <- 7
  # loop :a {
  #   u <- shl(u, 8)
  #   u <- or(u, read1(add(at, i)))
  #   if eq(i, 0) {
  #     break :a
  #   }
  #   i <- sub(i, 1)
  # }

  u <- or(u, read1(add(at, 7)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 6)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 5)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 4)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 3)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 2)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 1)))
  u <- shl(u, 8)
  u <- or(u, read1(add(at, 0)))
}

# Function to store a 64-bit value using the little-endian (LE) convention.
# On a LE platform, this could be greatly simplified using a cast.
def store64(at, u) {
  # i <- 0
  # loop :a {
  #   write1(add(at, i), u)
  #   u <- shr(u, 8)
  #   if eq(i, 7) {
  #     break :a
  #   }
  #   i <- add(i, 1)
  # }

  write1(add(at, 0), u)
  u <- shr(u, 8)
  write1(add(at, 1), u)
  u <- shr(u, 8)
  write1(add(at, 2), u)
  u <- shr(u, 8)
  write1(add(at, 3), u)
  u <- shr(u, 8)
  write1(add(at, 4), u)
  u <- shr(u, 8)
  write1(add(at, 5), u)
  u <- shr(u, 8)
  write1(add(at, 6), u)
  u <- shr(u, 8)
  write1(add(at, 7), u)
}

# load/store works correctly
# alloc(8)
# store64(0, 0x0123456789abcdef)
# store64(0, load64(0))
# trap(load64(0))

# # Function to XOR into a 64-bit value using the little-endian (LE) convention.
# # On a LE platform, this could be greatly simplified using a cast.
def xor64(at, u) {
  # i <- 0
  # loop :a {
  #   write1(add(at, i), xor(u, read1(add(at, i))))
  #   u <- shr(u, 8)
  #   if eq(i, 7) {
  #     break :a
  #   }
  #   i <- add(i, 1)
  # }

  write1(add(at, 0), xor(u, read1(add(at, 0))))
  u <- shr(u, 8)
  write1(add(at, 1), xor(u, read1(add(at, 1))))
  u <- shr(u, 8)
  write1(add(at, 2), xor(u, read1(add(at, 2))))
  u <- shr(u, 8)
  write1(add(at, 3), xor(u, read1(add(at, 3))))
  u <- shr(u, 8)
  write1(add(at, 4), xor(u, read1(add(at, 4))))
  u <- shr(u, 8)
  write1(add(at, 5), xor(u, read1(add(at, 5))))
  u <- shr(u, 8)
  write1(add(at, 6), xor(u, read1(add(at, 6))))
  u <- shr(u, 8)
  write1(add(at, 7), xor(u, read1(add(at, 7))))
}

def ROTL64(a, offset) -> n {
  # (a << offset) ^ (a >> (64-offset))
  n <- xor(shl(a, offset), shr(a, sub(64, offset)))
}

def i(x, y) -> n {
  # (x+5y)
  n <- add(x, mul(5, y))
}

def readLane(x, y) -> lane {
  lane <- load64(mul(8, i(x, y)))
}

def writeLane(x, y, lane) {
  store64(mul(8, i(x, y)), lane)
}

def XORLane(x, y, lane) {
  xor64(mul(8, i(x, y)), lane)
}

# alloc state (1600 bits)
alloc(200)

# alloc blocks
block_count <- add(1, div(dlen(), 136))
alloc(mul(136, block_count))

# temp array[5] space (C during theta step, temp during chi step)
temp_arr_ptr <- add(200, mul(136, block_count))
alloc(40)

# iota round constants
# TODO: Make these stack variables once I add arrays
round_constants_ptr <- add(40, temp_arr_ptr)
alloc(mul(24, 8))
write8(add(round_constants_ptr, mul(8, 0)), 0x0000000000000001)
write8(add(round_constants_ptr, mul(8, 1)), 0x0000000000008082)
write8(add(round_constants_ptr, mul(8, 2)), 0x800000000000808A)
write8(add(round_constants_ptr, mul(8, 3)), 0x8000000080008000)
write8(add(round_constants_ptr, mul(8, 4)), 0x000000000000808B)
write8(add(round_constants_ptr, mul(8, 5)), 0x0000000080000001)
write8(add(round_constants_ptr, mul(8, 6)), 0x8000000080008081)
write8(add(round_constants_ptr, mul(8, 7)), 0x8000000000008009)
write8(add(round_constants_ptr, mul(8, 8)), 0x000000000000008A)
write8(add(round_constants_ptr, mul(8, 9)), 0x0000000000000088)
write8(add(round_constants_ptr, mul(8, 10)), 0x0000000080008009)
write8(add(round_constants_ptr, mul(8, 11)), 0x000000008000000A)
write8(add(round_constants_ptr, mul(8, 12)), 0x000000008000808B)
write8(add(round_constants_ptr, mul(8, 13)), 0x800000000000008B)
write8(add(round_constants_ptr, mul(8, 14)), 0x8000000000008089)
write8(add(round_constants_ptr, mul(8, 15)), 0x8000000000008003)
write8(add(round_constants_ptr, mul(8, 16)), 0x8000000000008002)
write8(add(round_constants_ptr, mul(8, 17)), 0x8000000000000080)
write8(add(round_constants_ptr, mul(8, 18)), 0x000000000000800A)
write8(add(round_constants_ptr, mul(8, 19)), 0x800000008000000A)
write8(add(round_constants_ptr, mul(8, 20)), 0x8000000080008081)
write8(add(round_constants_ptr, mul(8, 21)), 0x8000000000008080)
write8(add(round_constants_ptr, mul(8, 22)), 0x0000000080000001)
write8(add(round_constants_ptr, mul(8, 23)), 0x8000000080008008)

# copy data to memory
dcopy(0, 200, dlen())

# bits(input) + bits(0b01) + bits(0b1) + bits(0b1) + bits(n) + bits(0b1)
# write padding byte at the end of the copied input
write1(add(dlen(), 200), 0x06)
# write trailing 1 bit to end of blocks
write1(add(mul(136, block_count), 199), or(read1(add(mul(136, block_count), 199)), 0x80))

block_number <- 0
loop :blocks {
  if lt(block_number, block_count) {
    i <- 0
    loop :a {
      # 17 words per block
      if lt(i, 17) {
        state_word <- read8(mul(i, 8))
        # start of blocks (200) + block offset + word offset
        block_word <- read8(add(200, add(mul(block_number, 136), mul(i, 8))))
        write8(
          mul(i, 8),
          xor(state_word, block_word)
        )
        i <- add(i, 1)
      } else {
        break :a
      }
    }

    round <- 0
    x <- 0
    y <- 0
    j <- 0
    t <- 0
    loop :permute {
      if lt(round, 24) {
        # === θ step (see [Keccak Reference, Section 2.3.2]) ===

        C <- temp_arr_ptr

        # Compute the parity of the columns
        x <- 0
        loop :theta_parity {
          if lt(x, 5) {
            # write to C[x]
            write8(
              add(C, mul(x, 8)),
              xor(xor(xor(xor(readLane(x, 0), readLane(x, 1)), readLane(x, 2)), readLane(x, 3)), readLane(x, 4))
            )
            x <- add(x, 1)
          } else {
            break :theta_parity
          }
        }

        x <- 0
        D <- 0
        loop :theta_effect_x {
          if lt(x, 5) {
            # Compute the θ effect for a given column
            D <- xor(
              # C[(x+4)%5]
              read8(add(C, mul(mod(add(x, 4), 5), 8))),
              # C[(x+1)%5]
              ROTL64(read8(add(C, mul(mod(add(x, 1), 5), 8))), 1)
            )

            # Add the θ effect to the whole column
            y <- 0
            loop :theta_effect_y {
              if lt(y, 5) {
                XORLane(x, y, D)
                y <- add(y, 1)
              } else {
                break :theta_effect_y
              }
              # 1591140754
              # 1591810066
            }

            x <- add(x, 1)
          } else {
            break :theta_effect_x
          }
        }

        # === ρ and π steps (see [Keccak Reference, Sections 2.3.3 and 2.3.4]) ===
        current <- 0
        temp <- 0
        # Start at coordinates (1 0)
        x <- 1
        y <- 0
        current <- readLane(x, y)
        # Iterate over ((0 1)(2 3))^t * (1 0) for 0 ≤ t ≤ 23
        t <- 0
        r <- 0
        Y <- 0
        loop :rho_and_pi {
          if lt(t, 24) {
            # Compute the rotation constant r = (t+1)(t+2)/2
            r <- mod(div(mul(add(t, 1), add(t, 2)), 2), 64)
            # # Compute ((0 1)(2 3)) * (x y)
            Y <- mod(add(mul(2, x), mul(3, y)), 5)
            x <- y
            y <- Y
            # Swap current and state(x,y), and rotate
            temp <- readLane(x, y)
            writeLane(x, y, ROTL64(current, r))
            current <- temp
            t <- add(t, 1)
          } else {
            break :rho_and_pi
          }
        }

        # === χ step (see [Keccak Reference, Section 2.3.1]) ===
        y <- 0
        loop :chi {
          if lt(y, 5) {
            # Take a copy of the plane
            x <- 0
            loop :chi_copy_plane {
              if lt(x, 5) {
                write8(
                  add(temp_arr_ptr, mul(x, 8)),
                  readLane(x, y)
                )
                x <- add(x, 1)
              } else {
                break :chi_copy_plane
              }
            }
            # Compute χ on the plane
            x <- 0
            loop :chi_compute {
              if lt(x, 5) {
                writeLane(
                  x, y,
                  xor(
                    read8(add(temp_arr_ptr, mul(x, 8))),
                    and(
                      neg(read8(add(temp_arr_ptr, mul(mod(add(x, 1), 5), 8)))),
                          read8(add(temp_arr_ptr, mul(mod(add(x, 2), 5), 8)))
                    )
                  )
                )
                x <- add(x, 1)
              } else {
                break :chi_compute
              }
            }
            y <- add(y, 1)
          } else {
            break :chi
          }
        }

        # === ι step (see [Keccak Reference, Section 2.3.5]) ===
        XORLane(0, 0, read8(add(mul(8, round), round_constants_ptr)))
        round <- add(round, 1)
      } else {
        break :permute
      }
    }
  } else {
    break :blocks
  }
  block_number <- add(block_number, 1)
}

exit(0, 32)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = CheckCtx::new("");
        ctx.check(&ast).unwrap();

        let ast = Normalize::new().run(&ctx, ast);

        println!("{}", print_ast(&ast));
    }
}
