use chumsky::span::Spanned;

use crate::{
    mir::{
        CheckCtx, Visitor,
        ast::{self, BuiltinOrDef, Expr, Val},
    },
    op,
};

#[non_exhaustive]
pub struct ConstEval {}

impl ConstEval {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Visitor for ConstEval {
    fn visit_expr<'a>(&mut self, _: &CheckCtx, expr: &Expr<'a>) -> Option<Expr<'a>> {
        let res = const_eval(expr.clone());
        if &res == expr { None } else { Some(res) }
    }
}

fn const_eval<'a>(expr: Expr<'a>) -> Expr<'a> {
    match expr {
        Expr::Val(val) => Expr::Val(val),
        Expr::Var(var) => Expr::Var(var),
        Expr::Call { spread, f, args } => {
            use BuiltinOrDef::Builtin;
            use ast::Builtin::*;

            macro_rules! p_val {
                ($v:pat) => {
                    Expr::Val($v)
                };
            }

            // macro_rules! p_var {
            //     ($v:pat) => {
            //         Expr::Var($v)
            //     };
            // }

            macro_rules! p_call {
                ($pat:pat, $args:pat $(,)?) => {
                    Expr::Call { spread: false, f: $pat, args: $args }
                };
            }

            let len = args.len();

            let binop = |ctor: ast::Builtin,
                         op_f: fn(u64, u64) -> u64,
                         f_: fn(Expr<'a>, Expr<'a>) -> Option<Expr<'a>>|
             -> Expr<'a> {
                match (const_eval(args[0].clone()), const_eval(args[1].clone())) {
                    (p_val!(l), p_val!(r)) => Expr::val(op_f(l.value(), r.value()), f.span),
                    (l, r) => f_(l.clone(), r.clone()).unwrap_or_else(|| Expr::Call {
                        spread,
                        f: Spanned { inner: ctor.into(), span: f.span },
                        args: vec![l, r],
                    }),
                }
            };

            // eval the args first (depth first)
            match (&f.inner, len) {
                (Builtin(b), n) => match (b, n) {
                    (Add, 2) => binop(Add, op::add, |l, r| match (l, r) {
                        (p_val!(val), e) | (e, p_val!(val)) if val.value() == 0 => Some(e),
                        (
                            p_val!(l),
                            p_call!(
                                f @ Spanned { inner: Builtin(Add), .. },
                                args,
                            ),
                        )
                        | (
                            p_call!(
                                f @ Spanned { inner: Builtin(Add), .. },
                                args,
                            ),
                            p_val!(l),
                        ) => {
                            if let [p_val!(r), e] | [e, p_val!(r)] = &*args {
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
                    (Sub, 2) => binop(Sub, op::sub, |_, _| None),
                    (Mul, 2) => binop(Mul, op::mul, |_, _| None),
                    (Div, 2) => binop(Div, |a, b| op::div(a, b).unwrap(), |_, _| None),
                    (Exp, 2) => binop(Exp, op::expmod, |_, _| None),
                    (Mod, 2) => binop(Mod, |a, b| op::r#mod(a, b).unwrap(), |_, _| None),
                    (Eq, 2) => binop(Eq, op::eq, |_, _| None),
                    (Lt, 2) => binop(Lt, op::lt, |_, _| None),
                    (Gt, 2) => binop(Gt, op::gt, |_, _| None),
                    (Shl, 2) => binop(Shl, op::shl, |_, _| None),
                    (Shr, 2) => binop(Shr, op::shr, |_, _| None),
                    (Or, 2) => binop(Or, op::or, |_, _| None),
                    (Xor, 2) => binop(Xor, op::xor, |_, _| None),
                    (And, 2) => binop(And, op::and, |_, _| None),
                    (Not, 1) => match const_eval(args[0].clone()) {
                        p_val!(l) => Expr::val(op::not(l.value()), f.span),
                        e => Expr::Call {
                            spread,
                            f: Spanned { inner: Builtin(Not), span: f.span },
                            args: vec![e],
                        },
                    },
                    (Neg, 1) => match const_eval(args[0].clone()) {
                        p_val!(l) => Expr::val(op::neg(l.value()), f.span),
                        e => Expr::Call {
                            spread,
                            f: Spanned { inner: Builtin(Neg), span: f.span },
                            args: vec![e],
                        },
                    },
                    // same as default branch below
                    _ => Expr::Call { spread, f, args: args.into_iter().map(const_eval).collect() },
                },
                (_, _) => {
                    Expr::Call { spread, f, args: args.into_iter().map(const_eval).collect() }
                }
            }
        }
    }
}
