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
