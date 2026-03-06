use std::collections::BTreeMap;

use chumsky::span::Spanned;
use indexmap::IndexMap;
use tracing::{info_span, trace};

use crate::{
    assembler::{AsmOp, Object},
    mir::parse::{
        Assignment, Block, Break, Continue, Def, Else, Expr, Ident, If, Label, Loop, Statement,
    },
};

pub mod parse;

type Section<'a> = IndexMap<String, Vec<AsmOp<'a>>>;

#[derive(Debug)]
pub struct Ctx<'a> {
    prefix: String,
    sections: Section<'a>,
    fns: IndexMap<String, Section<'a>>,
    stack_depth: usize,
    scopes: Vec<Scope<'a>>,
}

#[derive(Debug)]
pub struct Scope<'a> {
    label: Option<Label<'a>>,
    /// var name -> stack index
    vars: BTreeMap<Ident<'a>, usize>,
    /// fn name -> label
    defs: BTreeMap<Ident<'a>, (Def<'a>, String)>,
}

impl<'a> Scope<'a> {
    fn drop_asm(&self) -> Vec<AsmOp<'a>> {
        let mut out = vec![];
        trace!(
            "popping scope {}",
            self.label.as_ref().map_or("<none>", |label| label.0.inner)
        );
        for (var, idx) in &self.vars {
            trace!("dropping var '{var}' @ idx {idx}");
            out.push(AsmOp::POP);
        }
        out
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    #[error("var '{var}' not found")]
    VarNotFound { var: String },
    #[error("def '{def}' not found")]
    DefNotFound { def: String },
    #[error(
        "builtin '{builtin}' can not be used as an expression as it does not return any values"
    )]
    StatementBuiltin { builtin: String },
    #[error("def '{def}' can not be used as an expression as it does not return any values")]
    StatementDef { def: String },
    #[error("builtin '{builtin}' takes {expected} argument(s), but {provided} were provided")]
    InvalidArgCountBuiltin {
        builtin: &'static str,
        expected: usize,
        provided: usize,
    },
    #[error("def '{def}' takes {expected} argument(s), but {provided} were provided")]
    InvalidArgCountDef {
        def: String,
        expected: usize,
        provided: usize,
    },
    #[error("'{def}' returns more than one value, ... must be used on the expression")]
    SpreadRequired { def: String },
    #[error("'{def}' does not return more than one value, ... can not be used")]
    InvalidSpread { def: String },
    #[error("... is only used on args")]
    SpreadTopLevel {},
}

pub type CompileResult<T = ()> = Result<T, CompileError>;

macro_rules! bug {
    ($($tt:tt)*) => {
        #[allow(clippy::panic)]
        { panic!($($tt)*) }
    };
}

impl<'a> Ctx<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new_root() -> Self {
        Ctx::new("MAIN")
    }

    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_owned(),
            sections: [(format!("$ROOT/{prefix}"), vec![])].into_iter().collect(),
            stack_depth: 0,
            fns: Default::default(),
            scopes: [Scope {
                label: Some(Label(Spanned {
                    // TODO: Figure out a better way to do this
                    inner: format!("$ROOT/{prefix}").leak(),
                    span: (0..0).into(),
                })),
                vars: Default::default(),
                defs: Default::default(),
            }]
            .into_iter()
            .collect(),
        }
    }

    pub fn into_object(self) -> Object<'a> {
        let root_label = self
            .scopes
            .first()
            .unwrap()
            .label
            .as_ref()
            .unwrap()
            .to_owned()
            .0
            .inner
            .into();

        Object(
            [("@start".into(), vec![AsmOp::PUSHL(root_label), AsmOp::JUMP])]
                .into_iter()
                .chain(self.fns.into_iter().flat_map(|(_, x)| x))
                .chain(self.sections)
                .map(|x| (x.0.into(), x.1))
                .collect(),
        )
    }

    fn inc_stack(&mut self) {
        trace!("inc_stack {} -> {}", self.stack_depth, self.stack_depth + 1);
        self.stack_depth += 1;
    }

    #[track_caller]
    fn dec_stack(&mut self) {
        trace!("dec_stack {} -> {}", self.stack_depth, self.stack_depth - 1);

        self.stack_depth -= 1;
    }

    fn push_scope(&mut self, label: Option<Label<'a>>) {
        trace!(
            "pushing scope {}",
            label.map_or("<none>", |label| label.0.inner)
        );
        self.scopes.push(Scope {
            label,
            vars: Default::default(),
            defs: Default::default(),
        });
    }

    fn pop_scope(&mut self, label: Option<Label<'a>>) -> CompileResult {
        trace!(
            "pop_scope {}",
            label.map_or("<none>", |label| label.0.inner)
        );

        loop {
            match self.scopes.pop() {
                Some(scope) => {
                    trace!(
                        "popping scope {}",
                        scope.label.map_or("<none>", |label| label.0.inner)
                    );
                    for (var, _) in scope.vars {
                        trace!("popping var '{var}'");
                        self.dec_stack();
                    }
                    if label.is_none_or(|label| {
                        scope.label.is_some_and(|scope_label| scope_label == label)
                    }) {
                        return Ok(());
                    }
                }
                None => match label {
                    Some(label) => {
                        bug!(
                            "tried to exit out of named scope '{label}' but that scope does not exist in this context"
                        )
                    }
                    None => return Ok(()),
                },
            }
        }
    }

    fn cleanup_scopes_to_label(&mut self, label: Label<'a>, salt: &str) {
        trace!("scope_cleanup_asm {}", label);

        for scope in self.scopes.iter().rev() {
            tracing::info!(
                "appending drop asm for scope {}",
                scope.label.map_or("<none>", |label| label.0.inner)
            );

            let key = format!(
                "{}:drop::{label}::{salt}::{}",
                self.prefix,
                scope.label.map_or("<none>", |label| label.0.inner)
            );

            self.sections.insert(key, scope.drop_asm());

            if scope
                .label
                .as_ref()
                .is_some_and(|scope_label| *scope_label == label)
            {
                return;
            }
        }
    }

    fn get_var(&self, var: Ident<'a>) -> Option<usize> {
        self.scopes.iter().find_map(|s| {
            s.vars
                .iter()
                .find_map(|(v, i)| v.0.inner.eq(var.0.inner).then_some(*i))
        })
    }

    fn init_var<'b>(&'b mut self, var: Ident<'a>) -> usize {
        self.init_var_with_depth_offset(var, 0)
    }

    fn init_var_with_depth_offset(&mut self, var: Ident<'a>, depth: isize) -> usize {
        let var_idx = self.stack_depth.strict_add_signed(depth);
        self.scopes
            .last_mut()
            .expect("no scopes?")
            .vars
            .insert(var, var_idx);
        // self.inc_stack();
        var_idx
    }

    fn get_def(&self, def: Ident<'a>) -> Option<&(Def<'a>, String)> {
        self.scopes.iter().find_map(|s| {
            s.defs
                .iter()
                .find_map(|(d, i)| d.0.inner.eq(def.0.inner).then_some(i))
        })
    }

    fn current_scope(&mut self) -> &mut Scope<'a> {
        self.scopes.last_mut().expect("main scope exists")
    }

    #[track_caller]
    fn push_section(&mut self, section_label: &str) -> &mut Vec<AsmOp<'a>> {
        self.sections.insert(section_label.to_owned(), vec![]);
        &mut self.sections[section_label]
    }

    #[track_caller]
    fn current_section(&mut self) -> &mut Vec<AsmOp<'a>> {
        self.sections.last_mut().expect("main section exists").1
    }

    #[track_caller]
    fn find_labelled_section(&mut self, label: Label<'a>) -> Option<Label<'a>> {
        self.scopes
            .iter()
            .rfind(|scope| scope.label == Some(label))
            .map(|scope| scope.label.unwrap())
    }

    fn loop_start_label(&self, label: Label<'_>) -> String {
        format!("{}:loop_start_{label}_[{}]", self.prefix, label.0.span)
    }

    fn loop_end_label(&self, label: Label<'_>) -> String {
        format!("{}:loop_end_{label}_[{}]", self.prefix, label.0.span)
    }
}

pub fn compile<'a: 'b, 'b>(ctx: &mut Ctx<'a>, block: &'b Block<'a>) -> CompileResult {
    fn go<'a: 'b, 'b>(ctx: &mut Ctx<'a>, depth: usize, block: &'b Block<'a>) -> CompileResult {
        let stack_depth_before = ctx.stack_depth;

        trace!(
            "go: {}",
            ctx.scopes
                .iter()
                .map(|s| s.label.map_or("<none>", |label| label.0.inner))
                .collect::<Vec<_>>()
                .join(",")
        );

        for (i, s) in block.0.iter().enumerate() {
            match s {
                Statement::Expr(expr) => {
                    trace!("expr");
                    let expr_ops = compile_expr(ctx, expr)?;
                    ctx.current_section().extend_from_slice(&expr_ops);
                }
                Statement::Loop(Loop(label, block)) => {
                    trace!("loop");
                    let loop_start_label = ctx.loop_start_label(*label);
                    let loop_end_label = ctx.loop_end_label(*label);
                    ctx.push_section(&loop_start_label);
                    ctx.push_scope(Some(*label));
                    go(ctx, depth + 1, block)?;
                    // append scope cleanup code just before jumping back to the beginning of the
                    // loop
                    ctx.cleanup_scopes_to_label(*label, &format!("loop_exit_[{}]", label.0.span));
                    // exit scope
                    ctx.pop_scope(Some(*label))?;
                    ctx.current_section()
                        // force non-zero jump
                        .extend_from_slice(&[AsmOp::PUSHL(loop_start_label.into()), AsmOp::JUMP]);
                    ctx.push_section(&loop_end_label);
                }
                Statement::Break(Break(label)) => {
                    trace!("break");

                    let dest_label = ctx.find_labelled_section(*label).unwrap();

                    let loop_end_label = ctx.loop_end_label(dest_label);

                    // append scope cleanup code just before exiting the loop
                    ctx.cleanup_scopes_to_label(*label, &format!("loop_break_[{}]", label.0.span));

                    trace!("cleaned up scope '{label}'");

                    ctx.current_section()
                        .extend_from_slice(&[AsmOp::PUSHL(loop_end_label.into()), AsmOp::JUMP]);
                }
                Statement::Continue(Continue(label)) => {
                    trace!("continue");

                    let dest_label = ctx.find_labelled_section(*label).unwrap();

                    let loop_start_label = ctx.loop_start_label(dest_label);

                    // append scope cleanup code just before jumping back to the beginning of the
                    // loop
                    ctx.cleanup_scopes_to_label(
                        *label,
                        &format!("loop_continue_[{}]", label.0.span),
                    );

                    ctx.current_section()
                        // force non-zero jump
                        .extend_from_slice(&[AsmOp::PUSHL(loop_start_label.into()), AsmOp::JUMP]);
                }
                Statement::If(if_) => {
                    fn go_if<'a>(
                        ctx: &mut Ctx<'a>,
                        If { cond, block, else_ }: If<'a>,
                        depth: usize,
                    ) -> CompileResult {
                        let (if_false_label, end_label_if_tail) = match &else_ {
                            Some(else_) => match else_ {
                                Else::ElseIf { if_ } => (
                                    format!("{}:if_cond_[{}]", ctx.prefix, if_.cond.span()),
                                    None,
                                ),
                                Else::Tail { block } => {
                                    // on false, if the next block is a tail else block, then jump
                                    // to the start of the tail block
                                    (
                                        format!("{}:if_tail_block_[{}]", ctx.prefix, block.0.span),
                                        Some(format!(
                                            "{}:if_tail_end_[{}]",
                                            ctx.prefix, block.0.span
                                        )),
                                    )
                                }
                            },
                            None => (format!("{}:if_end_[{}]", ctx.prefix, cond.span()), None),
                        };

                        let if_cond_label = format!("{}:if_cond_[{}]", ctx.prefix, cond.span());
                        ctx.push_section(&if_cond_label);

                        trace!("if {if_cond_label}");

                        // evaluate condition expression
                        let mut cond_asm = compile_expr(ctx, &cond)?;

                        // jump to end of the if statement (past the block code) if the expr is
                        // false
                        cond_asm.extend_from_slice(&[
                            AsmOp::NOT,
                            AsmOp::PUSHL(if_false_label.clone().into()),
                            AsmOp::JNZ,
                        ]);
                        ctx.dec_stack();
                        ctx.current_section().extend_from_slice(&cond_asm);

                        ctx.push_section(&format!("{}:if_block_[{}]", ctx.prefix, block.0.span));

                        ctx.push_scope(None);
                        go(ctx, depth + 1, &block)?;
                        ctx.pop_scope(None)?;

                        if let Some(end_label) = end_label_if_tail {
                            ctx.current_section()
                                .extend([AsmOp::PUSHL(end_label.into()), AsmOp::JUMP]);
                        }

                        match else_ {
                            Some(else_) => match else_ {
                                Else::ElseIf { if_ } => {
                                    trace!("else if");
                                    go_if(ctx, if_.inner, depth + 1)?
                                }
                                Else::Tail { block } => {
                                    let tail_end_label =
                                        format!("{}:if_tail_end_[{}]", ctx.prefix, block.0.span);
                                    let tail_block_label =
                                        format!("{}:if_tail_block_[{}]", ctx.prefix, block.0.span);
                                    trace!("else");
                                    ctx.push_section(&tail_block_label);
                                    go(ctx, depth + 1, &block)?;
                                    ctx.push_section(&tail_end_label);
                                }
                            },
                            None => {
                                ctx.push_section(&if_false_label);
                            }
                        }

                        Ok(())
                    }

                    trace!("if");

                    go_if(ctx, if_.clone(), depth)?;
                }
                Statement::Assignment(Assignment(vars, expr)) => {
                    let arity = expr_arity(ctx, 0, expr)?;
                    assert_eq!(vars.len(), arity);

                    // def f() -> a, b, c {}
                    // d, e, f <- f()
                    // # pushed to the stack in this order:
                    // # [c, b, a]

                    // if any vars on the lhs are updates, then init any newly declared vars first
                    // before evaluating the rhs
                    if vars.iter().any(|v| ctx.get_var(*v).is_some()) {
                        for (i, var) in vars.iter().rev().enumerate() {
                            if ctx.get_var(*var).is_none() {
                                trace!("var decl '{var}' (i: {i}) [pre-init]");
                                let idx = ctx.init_var(*var);
                                ctx.inc_stack();
                                // init the value to 0
                                ctx.current_section().push(AsmOp::push(0));
                                trace!("idx = {idx}");
                            }
                        }
                    }

                    // evaluate the expression
                    let expr_ops = compile_expr(ctx, expr)?;
                    ctx.current_section().extend_from_slice(&expr_ops);

                    for (i, var) in vars.iter().rev().enumerate() {
                        match ctx.get_var(*var) {
                            // var declaration, initial value was already pushed to the stack above
                            // when evaluating the rhs expression, so just store the variable's
                            // stack position
                            None => {
                                trace!("var decl '{var}' (i: {i})");
                                let idx =
                                    ctx.init_var_with_depth_offset(*var, dbg!(-((i + 1) as isize)));
                                trace!("idx = {idx}");
                            }
                            // var already declared, update it's value by evaluating the expression
                            // and swapping the old value with the new
                            // one, and then popping the old value
                            Some(var_stack_idx) => {
                                trace!(
                                    "var update '{var}' (i: {i}, var_stack_idx: {var_stack_idx}, stack_depth: {})",
                                    ctx.stack_depth
                                );
                                // TODO: Figure why this is -2 lol
                                let stack_location_from_top = (ctx.stack_depth - var_stack_idx) - 2;
                                trace!("stack_location_from_top: {stack_location_from_top}");
                                ctx.current_section().extend_from_slice(&[
                                    AsmOp::push(stack_location_from_top as u64),
                                    AsmOp::SWAP,
                                    AsmOp::POP,
                                ]);
                                ctx.dec_stack();
                            }
                        }
                    }
                }
                Statement::Def(def) => info_span!("def").in_scope(|| -> CompileResult {
                    // // args.len() + 1 for return pointer
                    // assert!(ctx.stack_depth > def.args.len());

                    let def_label = format!("{}:def_{}_{depth}_{i}", ctx.prefix, def.ident);
                    // this function is callable in this scope
                    ctx.current_scope()
                        .defs
                        .insert(def.ident, (def.clone(), def_label.clone()));

                    let mut def_ctx = Ctx::new(&format!("{}/{def_label}", ctx.prefix));
                    def_ctx.push_section(&def_label);

                    // calling convention is [...args, @caller_ptr, ...rets]
                    // args will be popped before returning
                    // output is [...rets]
                    // therefore, before calling the final JUMP op, the stack must be
                    // [...rets, @caller_ptr]

                    // args are provided by the caller, init them in the new ctx
                    for arg in &def.args {
                        trace!("arg '{arg}'");
                        def_ctx.init_var(*arg);
                        def_ctx.inc_stack();
                    }

                    // account for @caller_ptr, also provided by the caller
                    // NOTE: The return pointer is pushed at the callsite by CALL
                    trace!("@caller_ptr");
                    def_ctx.inc_stack();

                    // new ctx values for this fn call

                    def_ctx
                        .sections
                        .insert(format!("{def_label}/RETS_INIT"), vec![]);

                    // init return values
                    for ret in def.rets.iter().rev() {
                        trace!("ret '{ret}'");
                        def_ctx.init_var(*ret);
                        def_ctx.inc_stack();
                        def_ctx.current_section().push(AsmOp::push(0));
                    }

                    // functions can access other functions visible in this scope
                    for (def_name, label) in ctx.scopes.iter().flat_map(|s| &s.defs) {
                        def_ctx
                            .current_scope()
                            .defs
                            .insert(*def_name, label.clone());
                    }

                    def_ctx.push_section(&format!("{def_label}/BODY"));

                    // compile the fn body
                    go(&mut def_ctx, depth + 1, &def.body)?;

                    def_ctx
                        .sections
                        .insert(format!("{def_label}/CLEANUP"), vec![]);

                    // go from [...args, @caller_ptr, ...rets] to [...rets, @caller_ptr, ...args]
                    def_ctx
                        .current_section()
                        .extend(reverse_list_ops(def.args.len() + 1 + def.rets.len()));

                    for arg in &def.args {
                        trace!("arg pop '{arg}'");
                        def_ctx.current_section().extend_from_slice(&[AsmOp::POP]);
                        def_ctx.dec_stack();
                    }

                    def_ctx.current_section().extend([AsmOp::JUMP]);

                    ctx.fns.insert(def_label, def_ctx.sections);

                    Ok(())
                })?,
            }
        }

        trace!(
            "go end: {}",
            ctx.scopes
                .iter()
                .map(|s| s.label.map_or("<none>", |label| label.0.inner))
                .collect::<Vec<_>>()
                .join(",")
        );

        let stack_depth_after = ctx.stack_depth;

        trace!("stack_depth_before: {stack_depth_before}, stack_depth_after: {stack_depth_after}");

        Ok(())
    }

    go(ctx, 0, block)
}

fn exprs_arity<'a>(ctx: &Ctx<'a>, depth: usize, exprs: &[Expr<'_>]) -> CompileResult<usize> {
    exprs
        .iter()
        .map(|expr| expr_arity(ctx, depth, expr))
        .sum::<CompileResult<usize>>()
}

fn expr_arity<'a>(ctx: &Ctx<'a>, depth: usize, expr: &Expr<'_>) -> CompileResult<usize> {
    match expr {
        Expr::Val(_) | Expr::Var(_) => Ok(1),
        Expr::Call {
            spread,
            f: builtin,
            args: _,
        } if matches!(
            builtin.0.inner,
            "add" | "mul" | "sub" | "exp" | "mod" | "eq" | "lt" | "gt" | "dread1" | "dlen"
        ) =>
        {
            if depth > 0 && *spread {
                Err(CompileError::InvalidSpread {
                    def: builtin.0.inner.to_owned(),
                })
            } else {
                Ok(1)
            }
        }
        Expr::Call {
            spread,
            f: builtin,
            args: _,
        } if matches!(
            builtin.0.inner,
            "alloc" | "write1" | "write2" | "write8" | "exit" | "trap"
        ) =>
        {
            if *spread {
                Err(CompileError::InvalidSpread {
                    def: builtin.0.inner.to_owned(),
                })
            } else {
                Err(CompileError::StatementBuiltin {
                    builtin: builtin.0.inner.to_owned(),
                })
            }
        }
        Expr::Call {
            spread,
            f: def,
            args: _,
        } => {
            let arity = ctx
                .get_def(*def)
                .ok_or_else(|| CompileError::DefNotFound {
                    def: def.0.inner.to_owned(),
                })?
                .0
                .rets
                .len();

            match (depth, spread, arity) {
                (_, _, 0) => Err(CompileError::StatementDef {
                    def: def.0.inner.to_owned(),
                }),
                // '...' provided at top level, always invalid
                (0, true, _) => Err(CompileError::SpreadTopLevel {}),
                // '...' provided but only 1 return value
                (1.., true, 1) => Err(CompileError::InvalidSpread {
                    def: def.0.inner.to_owned(),
                }),
                // '...' not provided but more than 1 return value
                (1.., false, 2..) => Err(CompileError::SpreadRequired {
                    def: def.0.inner.to_owned(),
                }),
                _ => Ok(arity),
            }
        }
    }
}

// TODO: Modify ctx directly insted of returning the ops
fn compile_expr<'a>(ctx: &mut Ctx<'a>, expr: &Expr<'a>) -> CompileResult<Vec<AsmOp<'static>>> {
    fn go<'a>(
        ctx: &mut Ctx<'a>,
        depth: usize,
        ops: &mut Vec<AsmOp<'static>>,
        expr: &Expr<'a>,
    ) -> CompileResult {
        trace!("evaluating: {expr}");

        match expr {
            Expr::Val(val) => {
                trace!("val {val:#x}");
                ops.push(AsmOp::push(val.value()));
                ctx.inc_stack();
            }
            Expr::Var(var) => {
                let Some(idx) = ctx.get_var(*var) else {
                    return Err(CompileError::VarNotFound {
                        var: var.0.inner.to_owned(),
                    });
                };
                trace!("var '{var}' (idx: {idx}, depth: {})", ctx.stack_depth);
                // EXAMPLE:
                //
                // if the stack depth is 8, and the variable is at stack index 2, then the index
                // of the variable for the DUP op will be 5:
                //
                // 1 2 3 4 5 6 7 8 stack depth
                // 0 1 2 3 4 5 6 7 stack index
                // 7 6 5 4 3 2 1 0 DUP index
                //     ^
                //     var
                //
                // note that stack depth 1 == stack index 0
                trace!("stack_depth: {}", ctx.stack_depth);
                let dup_idx = (ctx.stack_depth - 1) - idx;
                trace!("dup_idx: {dup_idx}");
                ops.extend_from_slice(&[AsmOp::push(dup_idx as u64), AsmOp::DUP]);
                ctx.inc_stack();
            }
            Expr::Call {
                spread,
                f,
                args: exprs,
            } => {
                if depth == 0 && *spread {
                    return Err(CompileError::SpreadTopLevel {});
                }

                fn ensure_arity_and_eval_args<'a>(
                    ctx: &mut Ctx<'a>,
                    depth: usize,
                    ops: &mut Vec<AsmOp<'static>>,
                    builtin: &'static str,
                    expected: usize,
                    exprs: &[Expr<'a>],
                ) -> CompileResult {
                    trace!("{builtin}");
                    let arity = exprs_arity(ctx, depth + 1, exprs)?;
                    if arity != expected {
                        Err(CompileError::InvalidArgCountBuiltin {
                            builtin,
                            expected,
                            provided: arity,
                        })
                    } else {
                        for expr in exprs.iter() {
                            go(ctx, depth + 1, ops, expr)?;
                        }
                        Ok(())
                    }
                }

                match f.0.inner {
                    "add" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "add", 2, exprs)?;
                        ops.push(AsmOp::ADD);
                        ctx.dec_stack();
                    }
                    "mul" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "mul", 2, exprs)?;
                        ops.push(AsmOp::MUL);
                        ctx.dec_stack();
                    }
                    "sub" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "sub", 2, exprs)?;
                        ops.push(AsmOp::SUB);
                        ctx.dec_stack();
                    }
                    "exp" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "exp", 2, exprs)?;
                        ops.push(AsmOp::EXP);
                        ctx.dec_stack();
                    }
                    "mod" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "mod", 2, exprs)?;
                        ops.push(AsmOp::MOD);
                        ctx.dec_stack();
                    }
                    "eq" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "eq", 2, exprs)?;
                        ops.push(AsmOp::EQ);
                        ctx.dec_stack();
                    }
                    "lt" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "lt", 2, exprs)?;
                        ops.push(AsmOp::LT);
                        ctx.dec_stack();
                    }
                    "gt" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "gt", 2, exprs)?;
                        ops.push(AsmOp::GT);
                        ctx.dec_stack();
                    }
                    "alloc" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "alloc", 1, exprs)?;
                        ops.push(AsmOp::ALLOC);
                        ctx.dec_stack();
                    }
                    "write1" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "write1", 2, exprs)?;
                        ops.push(AsmOp::WRITE1);
                        ctx.dec_stack();
                        ctx.dec_stack();
                    }
                    "write2" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "write2", 2, exprs)?;
                        ops.push(AsmOp::WRITE2);
                        ctx.dec_stack();
                        ctx.dec_stack();
                    }
                    "write8" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "write8", 2, exprs)?;
                        ops.push(AsmOp::WRITE8);
                        ctx.dec_stack();
                        ctx.dec_stack();
                    }
                    "dread1" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "dread1", 1, exprs)?;
                        ops.push(AsmOp::DREAD1);
                    }
                    "dlen" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "dlen", 0, exprs)?;
                        ops.push(AsmOp::DLEN);
                        ctx.inc_stack();
                    }
                    "exit" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "exit", 2, exprs)?;
                        ops.push(AsmOp::EXIT);
                        ctx.dec_stack();
                        ctx.dec_stack();
                    }
                    "trap" => {
                        ensure_arity_and_eval_args(ctx, depth, ops, "trap", 1, exprs)?;
                        ops.push(AsmOp::TRAP);
                        ctx.dec_stack();
                    }
                    _ => {
                        trace!("call '{f}'");

                        let (def, def_label) = ctx.get_def(*f).expect("def not found").clone();

                        if exprs_arity(ctx, depth + 1, exprs)? != def.args.len() {
                            return Err(CompileError::InvalidArgCountDef {
                                def: def.ident.0.inner.to_owned(),
                                expected: def.args.len(),
                                provided: exprs.len(),
                            });
                        }

                        ctx.push_scope(None);
                        let mut args = def.args.clone();
                        args.reverse();
                        for expr in exprs.iter() {
                            #[allow(clippy::unwrap_in_result)]
                            for _ in
                                0..expr_arity(ctx, depth + 1, expr).expect("checked above; qed;")
                            {
                                let arg = args.pop().expect("checked above; qed;");
                                trace!("arg init '{arg}'");
                                ctx.init_var(arg);
                            }
                            go(ctx, depth + 1, ops, expr)?;
                        }

                        // all args are dropped from the stack
                        ctx.pop_scope(None)?;

                        ops.extend([AsmOp::PUSHL(def_label.into()), AsmOp::CALL]);

                        // all return values are pushed to the stack
                        for ret in &def.rets {
                            trace!("initing var {ret}");
                            ctx.inc_stack();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    let mut out = vec![];

    go(ctx, 0, &mut out, expr)?;

    Ok(out)
}

fn reverse_list_ops(list_len: usize) -> Vec<AsmOp<'static>> {
    let list_len = list_len as u64;
    // for 5:
    //  3 2 1 0
    // [A B C D E] SWAP 3
    // [E B C D A] SWAP 2
    // [E A C D B] SWAP 0
    // [E A C B D] SWAP 2
    // [E D C B A]
    //
    // for 6:
    //  4 3 2 1 0
    // [A B C D E F]
    //  ^         ^   SWAP 4
    // [F B C D E A]
    //          ^ ^   SWAP 0
    // [F B C D A E]
    //    ^       ^   SWAP 3
    // [F E C D A B]
    //        ^   ^   SWAP 1
    // [F E C B A D]
    //      ^     ^   SWAP 2
    // [F E D B A C]
    //        ^   ^   SWAP 1
    // [F E D C A B]
    //          ^ ^   SWAP 0
    // [F E D C B A]
    //
    // for 9:
    //  7 6 5 4 3 2 1 0
    // [A B C D E F G H I]
    //  ^               ^   SWAP 7
    // [I B C D E F G H A]
    //                ^ ^   SWAP 0
    // [I B C D E F G A H]
    //    ^             ^   SWAP 6
    // [I H C D E F G A B]
    //              ^   ^   SWAP 1
    // [I H C D E F B A G]
    //      ^           ^   SWAP 5
    // [I H G D E F B A C]
    //            ^     ^   SWAP 2
    // [I H G D E C B A F]
    //        ^         ^   SWAP 4
    // [I H G F E C B A D]
    //            ^     ^   SWAP 2
    // [I H G F E D B A C]
    //              ^   ^   SWAP 1
    // [I H G F E D C A B]
    //                ^ ^   SWAP 0
    // [I H G F E D C B A]

    match list_len {
        0 | 1 => vec![],
        2 => vec![AsmOp::push(0), AsmOp::SWAP],
        3 => vec![AsmOp::push(1), AsmOp::SWAP],
        _ => {
            let max_idx = list_len - 2;
            let mut ops = vec![];

            for idx in ((max_idx.div_ceil(2) + 1)..=max_idx).rev() {
                ops.extend([AsmOp::push(idx), AsmOp::SWAP]);
                ops.extend([AsmOp::push(max_idx - idx), AsmOp::SWAP]);
            }

            ops.extend([AsmOp::push(max_idx.div_ceil(2)), AsmOp::SWAP]);

            for idx in (0..=((list_len / 2) - 2)).rev() {
                ops.extend([AsmOp::push(idx), AsmOp::SWAP]);
            }

            ops
        }
    }
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    use super::*;
    use crate::{Vm, mir::parse::grammar};

    #[test]
    fn reverse_list() {
        for mut list in [
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 6],
            vec![1, 2, 3, 4, 5, 6, 7],
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        ] {
            let ops = reverse_list_ops(list.len());
            dbg!(&ops);
            let mut vm = Vm::new(Object::from_ops(ops).assemble(), vec![]);
            vm.stack = list.clone();
            vm.run().unwrap();
            list.reverse();
            assert_eq!(vm.stack, list);
        }
    }

    #[test]
    fn compile_expr() {
        init();

        let raw = "
            var <- add(1, 2)
            var2 <- mul(4, add(var, 1))
            var <- add(var, var2)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, None);

        assert_eq!(
            vm.stack,
            [
                19, // var
                16, // var2
            ]
        );
    }

    fn init() {
        let _ = tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::filter::EnvFilter::from_default_env())
            .try_init();
    }

    #[test]
    fn compile_if() {
        init();

        let raw = "
            var <- 2
            var2 <- 10
            if eq(1, sub(var, 1)) {
                var <- add(var, var2)
            }
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, None);

        assert_eq!(
            vm.stack,
            [
                12, // var
                10  // var2
            ]
        );
    }

    #[test]
    fn compile_if_else_if_branch() {
        init();

        let raw = "
            var <- 2
            var2 <- 10
            if 1 {
                var <- add(var, var2)
            } else {
                trap(1)
            }
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, None);

        assert_eq!(
            vm.stack,
            [
                12, // var
                10  // var2
            ]
        );
    }

    #[test]
    fn compile_if_else_else_branch() {
        init();

        let raw = "
            var <- 2
            var2 <- 10
            if eq(2, sub(var, 1)) {
                trap(1)
            } else {
                var <- add(var, var2)
            }
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, None);

        assert_eq!(
            vm.stack,
            [
                12, // var
                10  // var2
            ]
        );
    }

    #[test]
    fn compile_if_else_if() {
        init();

        let raw = "
            var <- 2
            var2 <- 10
            if 0 {
                trap(1)
            } else if 0 {
                trap(2)
            } else {
                var <- add(var, var2)
            }
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, None);

        assert_eq!(
            vm.stack,
            [
                12, // var
                10  // var2
            ]
        );
    }

    #[test]
    fn compile_def_single_arg() {
        init();

        let raw = "
            def square(i) -> o {
                o <- mul(i, i)
            }

            five <- add(1, 4)
            v <- square(five)

            u <- add(1, v)

            alloc(16)
            write8(0, v)
            write8(8, u)
            exit(0, 16)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(
            res,
            Some(
                [25_u64.to_be_bytes(), 26_u64.to_be_bytes()]
                    .as_flattened()
                    .to_vec()
            )
        );
    }

    #[test]
    fn compile_def_multiple_args() {
        init();

        let raw = "
            def add_mul(a, b) -> o {
                o <- mul(a, add(a, b))
            }

            three <- 3
            v <- add_mul(three, 5)

            alloc(8)
            write8(0, v)
            exit(0, 8)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(24_u64.to_be_bytes().to_vec()));
    }

    #[test]
    fn fib_recursive() {
        init();

        let raw = "
            def fib(n) -> m {
                if eq(n, 0) {
                    m <- 0
                }

                if eq(n, 1) {
                    m <- 1
                }

                if gt(n, 1) {
                    m <- add(fib(sub(n, 1)), fib(sub(n, 2)))
                }
            }

            res <- fib(10)

            alloc(8)
            write8(0, res)
            exit(0, 8)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(55_u64.to_be_bytes().to_vec()));
    }

    #[test]
    fn compile_def_shadowing() {
        init();

        let raw = "
            def digit_to_place(digit, idx) -> n {
              n <- mul(digit, exp(10, sub(dlen(), add(idx, 1))))
            }

            if eq(dlen(), 0) {
              trap(1)
            }

            n <- 0
            idx <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- add(n, digit_to_place(digit, idx))
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, n)
            exit(0, 8)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, b"123".to_vec());

        let res = vm.run().unwrap();

        assert_eq!(res, Some(123_u64.to_be_bytes().to_vec()));
    }

    #[test]
    fn multiple_return_values() {
        init();

        let raw = "
            def many(a) -> b, c, d, e, f {
                b <- add(a, 1)
                c <- add(a, 2)
                d <- add(a, 3)
                e <- add(a, 4)
                f <- add(a, 5)
            }

            a <- 100

            b, c, d, e, f <- many(a)

            alloc(6)
            write1(0, a)
            write1(1, b)
            write1(2, c)
            write1(3, d)
            write1(4, e)
            write1(5, f)
            exit(0, 6)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(vec![100, 101, 102, 103, 104, 105]));

        assert_eq!(vm.stack, [100, 101, 102, 103, 104, 105]);
    }

    #[test]
    fn multiple_return_values_update_and_init() {
        init();

        let raw = "
            def foo(a, b) -> c, d, e {
                c <- b
                d <- a
                e <- 0x22
            }

            a <- 0x11
            c <- 0x33

            a, b, c <- foo(a, c)

            alloc(3)
            write1(0, a)
            write1(1, b)
            write1(2, c)
            exit(0, 3)
        ";

        // # 0x33, 0x11, 0x22

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(
            res,
            Some(vec![
                0x33, // a
                0x11, // b
                0x22, // c
            ])
        );

        // a and c are pushed to the stack, then b when it is first set in the multi
        // assignment along with a and c being updated
        assert_eq!(
            vm.stack,
            [
                0x33, // a
                0x22, // c
                0x11, // b
            ]
        );
    }

    #[test]
    fn multiple_return_values_as_args() {
        init();

        let raw = "
            def foo(a) -> c, d {
                c <- mul(10, a)
                d <- mul(2, a)
            }

            a <- sub(...foo(4))

            alloc(1)
            write1(0, a)
            exit(0, 1)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(vec![(10 * 4) - (2 * 4)]));

        assert_eq!(vm.stack, [(10 * 4) - (2 * 4)]);
    }

    #[test]
    fn multiple_return_values_as_args_complex() {
        init();

        let raw = "
            def foo(a) -> c, d {
                c <- mul(10, a)
                d <- mul(2, a)
            }

            def bar(a, b, c) -> d {
                d <- mul(a, add(b, c))
            }

            def baz(a) -> d {
                d <- add(a, 1)
            }

            a <- 1
            res <- bar(...foo(4), a)

            alloc(3)
            write1(0, a)
            write2(1, res)
            exit(0, 3)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(
            res,
            Some(
                [[1_u8].as_slice(), 360_u16.to_be_bytes().as_slice()]
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect::<Vec<_>>()
            )
        );
    }

    #[test]
    fn multiple_return_swap_params() {
        init();

        let raw = "
            def swap(a_, b_) -> c, d {
                d <- a_
                c <- b_
            }

            a <- 0xaa
            b <- 0xbb

            a, b <- swap(a, b)

            alloc(2)
            write1(0, a)
            write1(1, b)
            exit(0, 2)
        ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(vec![0xbb, 0xaa]));

        assert_eq!(vm.stack, [0xbb, 0xaa]);
    }

    #[test]
    fn compile_loop() {
        init();

        let raw = "
            counter <- 0x00

            loop :a {
              counter <- add(counter, 1)
              if eq(counter, 10) {
                break :a
              }
            }

            alloc(1)
            write1(0, counter)
            exit(0, 1)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, vec![]);

        let res = vm.run().unwrap();

        assert_eq!(res, Some(vec![10]));
    }

    #[test]
    fn compile_loop_shadow_label() {
        init();

        let raw = "
            counter <- 0x00

            loop :a {
              loop :a {
                counter <- add(counter, 1)
                if eq(counter, 10) {
                  break :a
                }
              }
              break :a
            }

            alloc(1)
            write1(0, counter)
            exit(0, 1)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        dbg!(&obj);

        // let asm = obj.assemble();

        // let mut vm = Vm::new(asm, vec![]);

        // let res = vm.run().unwrap();

        // assert_eq!(res, Some(vec![10]));
    }

    #[test]
    fn compile_atoi() {
        // # n += digit * (10 ** (dlen() - (idx + 1)))
        let raw = "
            if eq(dlen(), 0) {
              trap(1)
            }

            n <- 0
            idx <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- add(n, mul(digit, exp(10, sub(dlen(), add(idx, 1)))))
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, n)
            exit(0, 8)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(asm, b"1234567".into());

        let res = vm.run().unwrap();

        assert_eq!(res.unwrap(), 1234567_u64.to_be_bytes());
    }

    #[test]
    fn compile_aoc_2025_1() {
        let raw = "
            if eq(dlen(), 0) {
              trap(1)
            }

            dial <- 100050
            total <- 0
            n <- 0
            idx <- 0
            is_right <- 0

            loop :a {
              if eq(dlen(), idx) {
                break :a
              }

              ascii_digit <- dread1(idx)

              if eq(ascii_digit, 76) {
                is_right <- 0

                idx <- add(idx, 1)
                n <- 0
                continue :a
              } else if eq(ascii_digit, 82) {
                is_right <- 1

                idx <- add(idx, 1)
                n <- 0
                continue :a
              } else if eq(ascii_digit, 10) {
                idx <- add(idx, 1)
                if is_right {
                  dial <- add(dial, n)
                  if eq(0, mod(dial, 100)) {
                    total <- add(total, 1)
                  }

                  continue :a
                }

                dial <- sub(dial, n)
                if eq(0, mod(dial, 100)) {
                  total <- add(total, 1)
                }

                continue :a
              }

              if lt(ascii_digit, 0x30) {
                trap(2)
              }

              if gt(ascii_digit, 0x39) {
                trap(3)
              }

              digit <- sub(ascii_digit, 0x30)
              n <- mul(n, 10)
              n <- add(n, digit)
              idx <- add(idx, 1)
            }

            alloc(8)
            write8(0, total)
            exit(0, 8)
            ";

        let ast = grammar().block.parse(raw).unwrap();

        let mut ctx = Ctx::new_root();

        compile(&mut ctx, &ast).unwrap();

        let obj = ctx.into_object();

        let asm = obj.assemble();

        let mut vm = Vm::new(
            asm,
            b"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
            .into(),
        );

        let res = vm.run().unwrap();

        assert_eq!(res, Some(3_u64.to_be_bytes().to_vec()))
    }
}
