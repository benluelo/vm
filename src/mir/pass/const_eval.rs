use chumsky::span::Spanned;

use crate::{
    mir::{
        CheckCtx,
        ast::{Assignment, Block, Builtin, BuiltinOrDef, Def, Expr, If, Loop, Statement, Val},
        pass::Pass,
    },
    op,
};

pub struct ConstEval {}

impl ConstEval {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Pass for ConstEval {
    #[expect(clippy::only_used_in_recursion)]
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

fn const_eval<'a>(expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Val(val) => Expr::Val(val),
        Expr::Var(var) => Expr::Var(var),
        Expr::Call { spread, f, args } => {
            use Builtin::*;

            let len = args.len();

            let binop = |ctor: Builtin,
                         op_f: fn(u64, u64) -> u64,
                         f_: fn(Expr<'a>, Expr<'a>) -> Option<Expr<'a>>|
             -> Expr<'a> {
                match (const_eval(args[0].clone()), const_eval(args[1].clone())) {
                    (Expr::Val(l), Expr::Val(r)) => Expr::Val(Val::new_spanned(Spanned {
                        inner: op_f(l.value(), r.value()),
                        span: f.span,
                    })),
                    (l, r) => f_(l.clone(), r.clone()).unwrap_or_else(|| Expr::Call {
                        spread,
                        f: Spanned {
                            inner: ctor.into(),
                            span: f.span,
                        },
                        args: vec![l, r],
                    }),
                }
            };

            match (f.inner, len) {
                (BuiltinOrDef::Builtin(Add), 2) => binop(Add, op::add, |l, r| match (l, r) {
                    (Expr::Val(val), e) | (e, Expr::Val(val)) if val.value() == 0 => Some(e),
                    (
                        Expr::Val(l),
                        Expr::Call {
                            spread: false,
                            f:
                                f @ Spanned {
                                    inner: BuiltinOrDef::Builtin(Builtin::Add),
                                    ..
                                },
                            args,
                        },
                    )
                    | (
                        Expr::Call {
                            spread: false,
                            f:
                                f @ Spanned {
                                    inner: BuiltinOrDef::Builtin(Builtin::Add),
                                    ..
                                },
                            args,
                        },
                        Expr::Val(l),
                    ) => {
                        if let [Expr::Val(r), e] | [e, Expr::Val(r)] = &*args {
                            Some(Expr::Call {
                                spread: false,
                                f,
                                args: vec![
                                    Expr::Val(Val::new(op::add(l.value(), r.value()))),
                                    e.clone(),
                                ],
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }),
                (BuiltinOrDef::Builtin(Sub), 2) => binop(Sub, op::sub, |_, _| None),
                (BuiltinOrDef::Builtin(Mul), 2) => binop(Mul, op::mul, |_, _| None),
                (BuiltinOrDef::Builtin(Div), 2) => {
                    binop(Div, |a, b| op::div(a, b).unwrap(), |_, _| None)
                }
                // Div => todo!(),
                // Exp => todo!(),
                (BuiltinOrDef::Builtin(Mod), 2) => {
                    binop(Mod, |a, b| op::r#mod(a, b).unwrap(), |_, _| None)
                }
                (BuiltinOrDef::Builtin(Eq), 2) => binop(Eq, op::eq, |_, _| None),
                (BuiltinOrDef::Builtin(Lt), 2) => binop(Lt, op::lt, |_, _| None),
                (BuiltinOrDef::Builtin(Gt), 2) => binop(Gt, op::gt, |_, _| None),
                // Shl => todo!(),
                // Shr => todo!(),
                (BuiltinOrDef::Builtin(Or), 2) => binop(Or, op::or, |_, _| None),
                (BuiltinOrDef::Builtin(Xor), 2) => binop(Xor, op::xor, |_, _| None),
                (BuiltinOrDef::Builtin(And), 2) => binop(And, op::and, |_, _| None),
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
