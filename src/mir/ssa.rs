use std::{collections::BTreeMap, fmt, iter};

use chumsky::{Parser, span::Spanned};
use indexmap::IndexMap;
use tracing::{info, info_span, instrument, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    assembler::AsmOp,
    mir::{
        CheckCtx, Id, IdentifiedLabel, LabelId, ScopeLabel,
        ast::{
            Assignment, Block, Break, Builtin, BuiltinOrDef, Continue, Def, Else, Expr, Ident, If,
            Label, Loop, Statement,
        },
        parse::grammar,
        pass::{Normalize, Pass},
        reverse_list_ops,
        ssa::builder::Builder,
    },
};

pub mod builder;
pub mod id_map;

macro_rules! bug {
    ($($tt:tt)*) => {
        #[allow(clippy::panic)]
        { panic!($($tt)*) }
    };
}

// COPIED FROM HERE ON

#[derive(Debug)]
pub struct Ctx<'a> {
    next_scope_label_id: LabelId,
    salt_id_counter: Id,
    prefix: String,
    sections: IndexMap<String, Vec<AsmOp<'a>>>,
    builder: Builder,
    fns: IndexMap<String, IndexMap<String, Vec<AsmOp<'a>>>>,
    stack_depth: usize,
    scopes: Vec<Scope<'a>>,
    counter: u32,
}

#[derive(Debug)]
pub struct Scope<'a> {
    #[expect(dead_code)]
    tag: String,
    label: ScopeLabel<'a>,
    /// var name -> stack index
    vars: BTreeMap<Ident<'a>, usize>,
    /// fn name -> label
    defs: BTreeMap<Ident<'a>, (Def<'a>, String)>,
}

impl<'a> Scope<'a> {
    fn drop_asm(&self) -> Vec<AsmOp<'a>> {
        let mut out = vec![];
        trace!("dropping vars in scope {}", self.label);
        for (var, idx) in &self.vars {
            trace!("dropping var '{var}' @ idx {idx}");
            out.push(AsmOp::POP);
        }
        out
    }
}

// pub struct VarInfo<'a> {
//     pub ident: Ident<'a>,
//     pub generation_counter: u32,
// }

#[derive(Debug, PartialEq, thiserror::Error)]
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
    #[error("label '{label}' not found")]
    LabelNotFound { label: String },
}

pub type CompileResult<T = ()> = Result<T, CompileError>;

impl<'a> Ctx<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new_root() -> Self {
        Ctx::new("MAIN")
    }

    pub fn new(prefix: &str) -> Self {
        let first_scope_label_id = LabelId::new();
        let identified_label = IdentifiedLabel::new(
            Label::new(
                // TODO: Figure out a better way to do this
                format!("$ROOT/{prefix}").leak(),
            ),
            first_scope_label_id,
        );
        let first_section_id = identified_label.to_string();
        let scope = Scope {
            tag: "root".to_owned(),
            label: ScopeLabel::Label(identified_label),
            vars: Default::default(),
            defs: Default::default(),
        };
        Self {
            next_scope_label_id: first_scope_label_id.increment(),
            salt_id_counter: Id::new(),
            prefix: prefix.to_owned(),
            sections: [(first_section_id.clone(), vec![])].into_iter().collect(),
            builder: Builder::new(),
            stack_depth: 0,
            fns: Default::default(),
            scopes: [scope].into_iter().collect(),
            counter: 0,
        }
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

    fn push_scope(&mut self, tag: String, label: ScopeLabel<'a>) {
        trace!("pushing scope {label} ({tag})",);
        self.scopes.push(Scope {
            tag,
            label,
            vars: Default::default(),
            defs: Default::default(),
        });
    }

    fn pop_scope(&mut self, label: ScopeLabel<'a>, cleanup_asm: bool) -> CompileResult {
        trace!("pop_scope {label}",);

        loop {
            match self.scopes.pop() {
                Some(scope) => {
                    trace!("popping scope {}", scope.label);
                    if cleanup_asm {
                        self.current_section().extend(scope.drop_asm());
                    }
                    for (var, _) in scope.vars {
                        trace!("popping var '{var}'");
                        self.dec_stack();
                    }
                    if label.matches_scope_label(&scope.label) {
                        return Ok(());
                    }
                }
                None => match label {
                    ScopeLabel::Label(label) => {
                        bug!(
                            "tried to exit out of named scope '{label}' but that scope does not exist in this context"
                        )
                    }
                    ScopeLabel::None => return Ok(()),
                },
            }
        }
    }

    fn get_salt(&mut self) -> Id {
        let res = self.salt_id_counter;
        self.salt_id_counter = self.salt_id_counter.increment();
        res
    }

    fn cleanup_scopes_to_label(&mut self, label: Label<'a>, from: &str) {
        trace!("scope_cleanup_asm {}", label);

        for scope in self.scopes.iter().rev() {
            tracing::info!("appending drop asm for scope {}", scope.label);

            let key = format!("{}:drop::{label}::{from}::{}", self.prefix, scope.label);

            self.sections.insert(key, scope.drop_asm());

            if scope.label.matches_label(&label) {
                return;
            }
        }
    }

    fn get_var(&self, var: &Ident<'a>) -> Option<usize> {
        self.scopes
            .iter()
            .find_map(|s| s.vars.iter().find_map(|(v, i)| v.eq(var).then_some(*i)))
    }

    fn init_var<'b>(&'b mut self, var: &Ident<'a>) -> usize {
        self.init_var_with_depth_offset(var, 0)
    }

    fn init_var_with_depth_offset(&mut self, var: &Ident<'a>, depth: isize) -> usize {
        info!("PUSHING VAR {var} @ {depth}");
        let var_idx = self.stack_depth.strict_add_signed(depth);
        self.scopes
            .last_mut()
            .expect("no scopes?")
            .vars
            .insert(var.clone(), var_idx);
        var_idx
    }

    fn get_def(&self, def: &Ident<'a>) -> Option<&(Def<'a>, String)> {
        self.scopes
            .iter()
            .find_map(|s| s.defs.iter().find_map(|(d, i)| d.eq(def).then_some(i)))
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
    fn push_block(
        &mut self,
        section_label: &str,
        args: Vec<String>,
        rets: Vec<String>,
    ) -> &mut Vec<TacStatement> {
        self.blocks.insert(
            section_label.to_owned(),
            BasicBlock {
                args,
                rets,
                body: vec![],
            },
        );
        &mut self.blocks[section_label].body
    }

    #[track_caller]
    fn current_section(&mut self) -> &mut Vec<AsmOp<'a>> {
        self.sections.last_mut().expect("main section exists").1
    }

    #[track_caller]
    fn current_block(&mut self) -> &mut Vec<TacStatement> {
        &mut self.blocks.last_mut().expect("main block exists").1.body
    }

    #[track_caller]
    fn find_labelled_section(&self, label: Label<'a>) -> Option<&IdentifiedLabel<'a>> {
        self.scopes
            .iter()
            .rfind(|scope| scope.label.matches_label(&label))
            .map(|scope| scope.label.as_label().unwrap())
    }

    fn loop_start_label(&self, label: &IdentifiedLabel<'_>) -> String {
        format!("{}:loop_start_{}:{}", self.prefix, label.label, label.id)
    }

    fn loop_end_label(&self, label: &IdentifiedLabel<'_>) -> String {
        format!("{}:loop_end_{}:{}", self.prefix, label.label, label.id)
    }

    pub fn compile<'b>(&mut self, block: &'b Block<'a>) -> CompileResult
    where
        'a: 'b,
    {
        fn go<'a: 'b, 'b>(ctx: &mut Ctx<'a>, depth: usize, block: &'b Block<'a>) -> CompileResult {
            let stack_depth_before = ctx.stack_depth;

            trace!(
                "go: {}",
                ctx.scopes
                    .iter()
                    .map(|s| s.label.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );

            for (i, s) in block.iter().enumerate() {
                match s {
                    Statement::Expr(expr) => {
                        trace!("expr");
                        let arity = ctx.expr_arity(0, expr, false)?;
                        ctx.compile_expr(expr)?;
                        ctx.current_section()
                            .extend(iter::repeat_n(AsmOp::POP, arity));
                        let iter = match ctx.normalize_tac_statmenets_from_ast_expr(expr.clone()) {
                            (_statements, SsaExpr::Const(_)) => todo!(),
                            (_statements, SsaExpr::Var(_)) => todo!(),
                            (statements, SsaExpr::Builtin(builtin_call)) => statements
                                .into_iter()
                                .chain([TacStatement::Builtin(builtin_call)]),
                        };
                        ctx.current_block().extend(iter);
                    }
                    Statement::Loop(Loop { label, block }) => {
                        trace!("loop");
                        let identified_label = ctx.new_label(*label);
                        let loop_start_label = ctx.loop_start_label(&identified_label);
                        let loop_end_label = ctx.loop_end_label(&identified_label);
                        ctx.push_section(&loop_start_label);
                        ctx.push_block(&loop_start_label, vec![], vec![]);
                        let scope_variable = ScopeLabel::Label(identified_label);
                        ctx.push_scope(format!("loop {label}"), scope_variable.clone());
                        go(ctx, depth + 1, block)?;
                        // append scope cleanup code just before jumping back to the beginning of
                        // the loop
                        ctx.cleanup_scopes_to_label(
                            *label,
                            &format!("loop_exit_[{identified_label}]"),
                        );
                        // exit scope
                        ctx.pop_scope(scope_variable, false)?;
                        ctx.current_section().extend_from_slice(&[
                            AsmOp::PUSHL(loop_start_label.clone().into()),
                            AsmOp::JUMP,
                        ]);
                        ctx.current_block().push(TacStatement::Goto {
                            label: loop_start_label,
                        });
                        ctx.push_section(&loop_end_label);
                        ctx.push_block(&loop_end_label, vec![], vec![]);
                    }
                    Statement::Break(Break(label)) => {
                        trace!("break");

                        let salt = ctx.get_salt();

                        let dest_label = ctx.find_labelled_section(*label).unwrap();

                        let loop_end_label = ctx.loop_end_label(dest_label);

                        // append scope cleanup code just before exiting the loop
                        ctx.cleanup_scopes_to_label(
                            *label,
                            &format!("loop_break_{dest_label}_{salt}"),
                        );

                        trace!("cleaned up scope '{label}'");

                        ctx.current_section().extend_from_slice(&[
                            AsmOp::PUSHL(loop_end_label.clone().into()),
                            AsmOp::JUMP,
                        ]);
                        ctx.current_block().push(TacStatement::Goto {
                            label: loop_end_label,
                        });
                    }
                    Statement::Continue(Continue(label)) => {
                        trace!("continue");

                        let salt = ctx.get_salt();

                        let dest_label = ctx.find_labelled_section(*label).unwrap();

                        let loop_start_label = ctx.loop_start_label(dest_label);

                        // append scope cleanup code just before jumping back to the beginning of
                        // the loop
                        ctx.cleanup_scopes_to_label(
                            *label,
                            &format!("loop_continue_{dest_label}_{salt}"),
                        );

                        ctx.current_section().extend_from_slice(&[
                            AsmOp::PUSHL(loop_start_label.clone().into()),
                            AsmOp::JUMP,
                        ]);
                        ctx.current_block().push(TacStatement::Goto {
                            label: loop_start_label,
                        });
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
                                        // on false, if the next block is a tail else block, then
                                        // jump to the start
                                        // of the tail block
                                        (
                                            format!(
                                                "{}:if_tail_block_[{}]",
                                                ctx.prefix,
                                                block.span()
                                            ),
                                            Some(format!(
                                                "{}:if_tail_end_[{}]",
                                                ctx.prefix,
                                                block.span()
                                            )),
                                        )
                                    }
                                },
                                None => (format!("{}:if_end_[{}]", ctx.prefix, cond.span()), None),
                            };

                            let if_cond_label = format!("{}:if_cond_[{}]", ctx.prefix, cond.span());
                            ctx.push_section(&if_cond_label);
                            ctx.push_block(&if_cond_label, vec![], vec![]);

                            trace!("if {if_cond_label}");

                            // evaluate condition expression
                            ctx.compile_expr(&cond)?;

                            // jump to end of the if statement (past the block code) if the expr is
                            // false
                            ctx.current_section().extend_from_slice(&[
                                AsmOp::NOT,
                                AsmOp::PUSHL(if_false_label.clone().into()),
                                AsmOp::JNZ,
                            ]);
                            ctx.dec_stack();
                            {
                                let (statements, cond) =
                                    ctx.normalize_tac_statmenets_from_ast_expr(cond);
                                ctx.current_block().extend(statements);
                                ctx.current_block().push(TacStatement::Jump {
                                    cond,
                                    label: if_false_label.clone(),
                                });
                            }

                            let label = format!("{}:if_block_[{}]", ctx.prefix, block.span());
                            ctx.push_section(&label);
                            ctx.push_block(&label, vec![], vec![]);

                            ctx.push_scope("if block".to_owned(), ScopeLabel::None);
                            go(ctx, depth + 1, &block)?;
                            ctx.pop_scope(ScopeLabel::None, true)?;

                            if let Some(end_label) = end_label_if_tail {
                                ctx.current_section()
                                    .extend([AsmOp::PUSHL(end_label.clone().into()), AsmOp::JUMP]);
                                ctx.current_block().push(TacStatement::Goto {
                                    label: end_label.clone(),
                                });
                            }

                            match else_ {
                                Some(else_) => match else_ {
                                    Else::ElseIf { if_ } => {
                                        trace!("else if");
                                        go_if(ctx, if_.inner, depth + 1)?
                                    }
                                    Else::Tail { block } => {
                                        let tail_end_label = format!(
                                            "{}:if_tail_end_[{}]",
                                            ctx.prefix,
                                            block.span()
                                        );
                                        let tail_block_label = format!(
                                            "{}:if_tail_block_[{}]",
                                            ctx.prefix,
                                            block.span()
                                        );
                                        trace!("else");
                                        ctx.push_section(&tail_block_label);
                                        ctx.push_block(&tail_block_label, vec![], vec![]);
                                        go(ctx, depth + 1, &block)?;
                                        ctx.push_section(&tail_end_label);
                                        ctx.push_block(&tail_end_label, vec![], vec![]);
                                    }
                                },
                                None => {
                                    ctx.push_section(&if_false_label);
                                    ctx.push_block(&if_false_label, vec![], vec![]);
                                }
                            }

                            Ok(())
                        }

                        trace!("if");

                        go_if(ctx, if_.clone(), depth)?;
                    }
                    Statement::Assignment(Assignment { vars, expr }) => {
                        let arity = ctx.expr_arity(0, expr, true)?;
                        assert_eq!(vars.len(), arity);

                        // def f() -> a, b, c {}
                        // d, e, f <- f()
                        // # pushed to the stack in this order:
                        // # [c, b, a]

                        // if any vars on the lhs are updates, then init any newly declared vars
                        // first before evaluating the rhs
                        if vars.iter().any(|v| ctx.get_var(v).is_some()) {
                            for (i, var) in vars.iter().rev().enumerate() {
                                if ctx.get_var(var).is_none() {
                                    trace!("var decl '{var}' (i: {i}) [pre-init]");
                                    let idx = ctx.init_var(var);
                                    ctx.inc_stack();
                                    // init the value to 0
                                    ctx.current_section().push(AsmOp::push(0));
                                    trace!("idx = {idx}");
                                }
                            }
                        }

                        // evaluate the expression
                        ctx.compile_expr(expr)?;

                        for (i, var) in vars.iter().rev().enumerate() {
                            match ctx.get_var(var) {
                                // var declaration, initial value was already pushed to the stack
                                // above when evaluating the rhs
                                // expression, so just store the variable's
                                // stack position
                                None => {
                                    trace!("var decl '{var}' (i: {i})");
                                    let idx =
                                        ctx.init_var_with_depth_offset(var, -((i + 1) as isize));
                                    trace!("idx = {idx}");
                                }
                                // var already declared, update it's value by evaluating the
                                // expression and swapping the old
                                // value with the new one, and then
                                // popping the old value
                                Some(var_stack_idx) => {
                                    trace!(
                                        "var update '{var}' (i: {i}, var_stack_idx: {var_stack_idx}, stack_depth: {})",
                                        ctx.stack_depth
                                    );
                                    // TODO: Figure why this is -2 lol
                                    let stack_location_from_top =
                                        (ctx.stack_depth - var_stack_idx) - 2;
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

                        assert_eq!(vars.len(), 1);
                        let (statements, expr) =
                            ctx.normalize_tac_statmenets_from_ast_expr(expr.clone());
                        ctx.current_block().extend(statements);
                        ctx.current_block().push(TacStatement::Assignment {
                            var: vars[0].to_string(),
                            expr,
                        });
                    }
                    Statement::Def(def) => {
                        info_span!("def", name = %def.ident).in_scope(|| -> CompileResult {
                            // // args.len() + 1 for return pointer
                            // assert!(ctx.stack_depth > def.args.len());

                            let def_label = format!("{}:def_{}_{depth}_{i}", ctx.prefix, def.ident);
                            // this function is callable in this scope
                            ctx.current_scope()
                                .defs
                                .insert(def.ident.clone(), (def.clone(), def_label.clone()));

                            let mut def_ctx = Ctx::new(&format!("{}/{def_label}", ctx.prefix));
                            def_ctx.push_section(&def_label);
                            def_ctx.push_block(&def_label, vec![], vec![]);

                            // calling convention is [...args, @caller_ptr, ...rets]
                            // args will be popped before returning
                            // output is [...rets]
                            // therefore, before calling the final JUMP op, the stack must be
                            // [...rets, @caller_ptr]

                            // args are provided by the caller, init them in the new ctx
                            for arg in &def.args {
                                trace!("arg '{arg}'");
                                def_ctx.init_var(arg);
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
                                def_ctx.init_var(ret);
                                def_ctx.inc_stack();
                                def_ctx.current_section().push(AsmOp::push(0));
                            }

                            // functions can access other functions visible in this scope
                            for (def_name, label) in ctx.scopes.iter().flat_map(|s| &s.defs) {
                                def_ctx
                                    .current_scope()
                                    .defs
                                    .insert(def_name.clone(), label.clone());
                            }

                            let label = format!("{def_label}/BODY");
                            def_ctx.push_section(&label);
                            def_ctx.push_block(&label, vec![], vec![]);

                            def_ctx
                                .push_scope(format!("def '{}' body", def.ident), ScopeLabel::None);
                            // compile the fn body
                            go(&mut def_ctx, depth + 1, &def.body)?;
                            def_ctx.pop_scope(ScopeLabel::None, true)?;

                            def_ctx
                                .sections
                                .insert(format!("{def_label}/CLEANUP"), vec![]);

                            // go from [...args, @caller_ptr, ...rets] to [...rets, @caller_ptr,
                            // ...args]
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
                        })?
                    }
                }
            }

            trace!(
                "go end: {}",
                ctx.scopes
                    .iter()
                    .map(|s| s.label.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );

            let stack_depth_after = ctx.stack_depth;

            trace!(
                "stack_depth_before: {stack_depth_before}, stack_depth_after: {stack_depth_after}"
            );

            Ok(())
        }

        // let root = self.cfg.add_node(Node::Root);

        go(self, 0, block)
    }

    fn exprs_arity(
        &self,
        depth: usize,
        exprs: &[Expr<'_>],
        ensure_expr: bool,
    ) -> CompileResult<usize> {
        exprs
            .iter()
            .map(|expr| self.expr_arity(depth, expr, ensure_expr))
            .sum::<CompileResult<usize>>()
    }

    fn expr_arity(&self, depth: usize, expr: &Expr<'_>, ensure_expr: bool) -> CompileResult<usize> {
        match expr {
            Expr::Val(_) | Expr::Var(_) => Ok(1),
            Expr::Call {
                spread,
                f:
                    Spanned {
                        inner: BuiltinOrDef::Builtin(builtin),
                        ..
                    },
                args: _,
            } if matches!(
                builtin,
                Builtin::Add
                    | Builtin::Sub
                    | Builtin::Mul
                    | Builtin::Div
                    | Builtin::Exp
                    | Builtin::Mod
                    | Builtin::Eq
                    | Builtin::Lt
                    | Builtin::Gt
                    | Builtin::Shl
                    | Builtin::Shr
                    | Builtin::Or
                    | Builtin::Xor
                    | Builtin::And
                    | Builtin::Not
                    | Builtin::Neg
                    | Builtin::Dread1
                    | Builtin::Dread2
                    | Builtin::Dread3
                    | Builtin::Dread4
                    | Builtin::Dread5
                    | Builtin::Dread6
                    | Builtin::Dread7
                    | Builtin::Dread8
                    | Builtin::Dlen
                    | Builtin::Read1
                    | Builtin::Read2
                    | Builtin::Read3
                    | Builtin::Read4
                    | Builtin::Read5
                    | Builtin::Read6
                    | Builtin::Read7
                    | Builtin::Read8
            ) =>
            {
                if depth > 0 && *spread {
                    Err(CompileError::InvalidSpread {
                        def: builtin.to_string(),
                    })
                } else {
                    Ok(1)
                }
            }
            Expr::Call {
                spread,
                f:
                    Spanned {
                        inner: BuiltinOrDef::Builtin(builtin),
                        ..
                    },
                args: _,
            } if matches!(
                builtin,
                Builtin::Alloc
                    | Builtin::Write1
                    | Builtin::Write2
                    | Builtin::Write3
                    | Builtin::Write4
                    | Builtin::Write5
                    | Builtin::Write6
                    | Builtin::Write7
                    | Builtin::Write8
                    | Builtin::Dcopy
                    | Builtin::Exit
                    | Builtin::Trap
            ) =>
            {
                if *spread {
                    Err(CompileError::InvalidSpread {
                        def: builtin.to_string(),
                    })
                } else if depth > 0 || ensure_expr {
                    Err(CompileError::StatementBuiltin {
                        builtin: builtin.to_string(),
                    })
                } else {
                    Ok(0)
                }
            }
            Expr::Call {
                spread,
                f: def,
                args: _,
            } => {
                let BuiltinOrDef::Def(def) = &def.inner else {
                    bug!("attempted to call builtin {} as a def", def.inner)
                };

                let arity = self
                    .get_def(def)
                    .ok_or_else(|| CompileError::DefNotFound {
                        def: def.to_string(),
                    })?
                    .0
                    .rets
                    .len();

                match (ensure_expr, depth, spread, arity) {
                    // statement def, invalid at top level if ensuring expression, invalid at any
                    // depth greater than top level, arity zero otherwise
                    (true, _, _, 0) | (_, 1.., _, 0) => Err(CompileError::StatementDef {
                        def: def.to_string(),
                    }),
                    (false, _, _, 0) => Ok(0),
                    // '...' provided at top level, always invalid
                    (_, 0, true, _) => Err(CompileError::SpreadTopLevel {}),
                    // '...' provided but only 1 return value
                    (_, 1.., true, 1) => Err(CompileError::InvalidSpread {
                        def: def.to_string(),
                    }),
                    // '...' not provided but more than 1 return value
                    (_, 1.., false, 2..) => Err(CompileError::SpreadRequired {
                        def: def.to_string(),
                    }),
                    _ => Ok(arity),
                }
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr<'a>) -> CompileResult<()> {
        #[instrument(level = "TRACE", skip_all, fields(%expr))]
        fn go<'a>(ctx: &mut Ctx<'a>, depth: usize, expr: &Expr<'a>) -> CompileResult {
            trace!("evaluating: {expr}");

            match expr {
                Expr::Val(val) => {
                    trace!("val {val:#x}");
                    ctx.current_section().push(AsmOp::push(val.value()));
                    ctx.inc_stack();
                }
                Expr::Var(var) => {
                    let Some(idx) = ctx.get_var(var) else {
                        return Err(CompileError::VarNotFound {
                            var: var.to_string(),
                        });
                    };
                    // dbg!(&ctx.scopes);
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
                    ctx.current_section()
                        .extend_from_slice(&[AsmOp::push(dup_idx as u64), AsmOp::DUP]);
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
                        builtin: &'static str,
                        expected: usize,
                        exprs: &[Expr<'a>],
                    ) -> CompileResult {
                        trace!("{builtin}");
                        let arity = ctx.exprs_arity(depth + 1, exprs, true)?;
                        if arity != expected {
                            Err(CompileError::InvalidArgCountBuiltin {
                                builtin,
                                expected,
                                provided: arity,
                            })
                        } else {
                            for expr in exprs.iter() {
                                go(ctx, depth + 1, expr)?;
                            }
                            Ok(())
                        }
                    }

                    match &f.inner {
                        BuiltinOrDef::Builtin(Builtin::Add) => {
                            ensure_arity_and_eval_args(ctx, depth, "add", 2, exprs)?;
                            ctx.current_section().push(AsmOp::ADD);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Mul) => {
                            ensure_arity_and_eval_args(ctx, depth, "mul", 2, exprs)?;
                            ctx.current_section().push(AsmOp::MUL);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Sub) => {
                            ensure_arity_and_eval_args(ctx, depth, "sub", 2, exprs)?;
                            ctx.current_section().push(AsmOp::SUB);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Div) => {
                            ensure_arity_and_eval_args(ctx, depth, "div", 2, exprs)?;
                            ctx.current_section().push(AsmOp::DIV);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Exp) => {
                            ensure_arity_and_eval_args(ctx, depth, "exp", 2, exprs)?;
                            ctx.current_section().push(AsmOp::EXP);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Mod) => {
                            ensure_arity_and_eval_args(ctx, depth, "mod", 2, exprs)?;
                            ctx.current_section().push(AsmOp::MOD);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Eq) => {
                            ensure_arity_and_eval_args(ctx, depth, "eq", 2, exprs)?;
                            ctx.current_section().push(AsmOp::EQ);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Lt) => {
                            ensure_arity_and_eval_args(ctx, depth, "lt", 2, exprs)?;
                            ctx.current_section().push(AsmOp::LT);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Gt) => {
                            ensure_arity_and_eval_args(ctx, depth, "gt", 2, exprs)?;
                            ctx.current_section().push(AsmOp::GT);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Shl) => {
                            ensure_arity_and_eval_args(ctx, depth, "shl", 2, exprs)?;
                            ctx.current_section().push(AsmOp::SHL);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Shr) => {
                            ensure_arity_and_eval_args(ctx, depth, "shr", 2, exprs)?;
                            ctx.current_section().push(AsmOp::SHR);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Or) => {
                            ensure_arity_and_eval_args(ctx, depth, "or", 2, exprs)?;
                            ctx.current_section().push(AsmOp::OR);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Xor) => {
                            ensure_arity_and_eval_args(ctx, depth, "xor", 2, exprs)?;
                            ctx.current_section().push(AsmOp::XOR);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::And) => {
                            ensure_arity_and_eval_args(ctx, depth, "and", 2, exprs)?;
                            ctx.current_section().push(AsmOp::AND);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Not) => {
                            ensure_arity_and_eval_args(ctx, depth, "not", 1, exprs)?;
                            ctx.current_section().push(AsmOp::NOT);
                        }
                        BuiltinOrDef::Builtin(Builtin::Neg) => {
                            ensure_arity_and_eval_args(ctx, depth, "neg", 1, exprs)?;
                            ctx.current_section().push(AsmOp::NEG);
                        }
                        BuiltinOrDef::Builtin(Builtin::Alloc) => {
                            ensure_arity_and_eval_args(ctx, depth, "alloc", 1, exprs)?;
                            ctx.current_section().push(AsmOp::ALLOC);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Write1) => {
                            ensure_arity_and_eval_args(ctx, depth, "write1", 2, exprs)?;
                            ctx.current_section().push(AsmOp::WRITE1);
                            ctx.dec_stack();
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Write2) => {
                            ensure_arity_and_eval_args(ctx, depth, "write2", 2, exprs)?;
                            ctx.current_section().push(AsmOp::WRITE2);
                            ctx.dec_stack();
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Write8) => {
                            ensure_arity_and_eval_args(ctx, depth, "write8", 2, exprs)?;
                            ctx.current_section().push(AsmOp::WRITE8);
                            ctx.dec_stack();
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Read1) => {
                            ensure_arity_and_eval_args(ctx, depth, "read1", 1, exprs)?;
                            ctx.current_section().push(AsmOp::READ1);
                        }
                        BuiltinOrDef::Builtin(Builtin::Read8) => {
                            ensure_arity_and_eval_args(ctx, depth, "read8", 1, exprs)?;
                            ctx.current_section().push(AsmOp::READ8);
                        }
                        BuiltinOrDef::Builtin(Builtin::Dread1) => {
                            ensure_arity_and_eval_args(ctx, depth, "dread1", 1, exprs)?;
                            ctx.current_section().push(AsmOp::DREAD1);
                        }
                        BuiltinOrDef::Builtin(Builtin::Dcopy) => {
                            ensure_arity_and_eval_args(ctx, depth, "dcopy", 3, exprs)?;
                            ctx.current_section().push(AsmOp::DCOPY);
                            ctx.dec_stack();
                            ctx.dec_stack();
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Dlen) => {
                            ensure_arity_and_eval_args(ctx, depth, "dlen", 0, exprs)?;
                            ctx.current_section().push(AsmOp::DLEN);
                            ctx.inc_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Exit) => {
                            ensure_arity_and_eval_args(ctx, depth, "exit", 2, exprs)?;
                            ctx.current_section().push(AsmOp::EXIT);
                            ctx.dec_stack();
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Builtin(Builtin::Trap) => {
                            ensure_arity_and_eval_args(ctx, depth, "trap", 1, exprs)?;
                            ctx.current_section().push(AsmOp::TRAP);
                            ctx.dec_stack();
                        }
                        BuiltinOrDef::Def(f) => {
                            trace!("call '{f}'");

                            let (def, def_label) = ctx.get_def(f).expect("def not found").clone();

                            if ctx.exprs_arity(depth + 1, exprs, true)? != def.args.len() {
                                return Err(CompileError::InvalidArgCountDef {
                                    def: def.ident.to_string(),
                                    expected: def.args.len(),
                                    provided: exprs.len(),
                                });
                            }

                            ctx.push_scope(format!("def '{}' args", def.ident), ScopeLabel::None);
                            let mut args = def.args.clone();
                            args.reverse();
                            // dbg!(*f, &args);
                            // evaluate all arg expressions to this call
                            //
                            // def f(a, b, c, d) {}
                            //
                            // f(x, ..y(), z)
                            //
                            // will evaluate as
                            //
                            // init a
                            // evaluate x
                            // init b
                            // init c
                            // evaluate ..y()
                            // init d
                            // evaluate z
                            for expr in exprs.iter() {
                                #[allow(clippy::unwrap_in_result)]
                                let arity = ctx
                                    .expr_arity(depth + 1, expr, true)
                                    .expect("checked above; qed;");

                                let tail = args.split_off(args.len() - arity);
                                trace!("evaluating args '{tail:?} from expr '{expr}'");

                                go(ctx, depth + 1, expr)?;
                            }

                            // dbg!(&ctx);

                            // all args are dropped from the stack
                            ctx.pop_scope(ScopeLabel::None, false)?;

                            ctx.current_section()
                                .extend([AsmOp::PUSHL(def_label.into()), AsmOp::CALL]);

                            for expr in exprs.iter() {
                                #[allow(clippy::unwrap_in_result)]
                                for _ in 0..ctx
                                    .expr_arity(depth + 1, expr, true)
                                    .expect("checked above; qed;")
                                {
                                    ctx.dec_stack();
                                }
                            }

                            // all return values are pushed to the stack
                            for ret in &def.rets {
                                trace!("initing var {ret}");
                                ctx.inc_stack();
                            }
                        }
                        _ => todo!(),
                    }
                }
            }

            Ok(())
        }

        go(self, 0, expr)
    }

    fn new_label(&mut self, label: Label<'a>) -> IdentifiedLabel<'a> {
        self.next_scope_label_id = self.next_scope_label_id.increment();
        IdentifiedLabel::new(label, self.next_scope_label_id)
    }

    fn normalize_tac_statmenets_from_ast_expr(
        &mut self,
        expr: Expr<'_>,
    ) -> (Vec<TacStatement>, SsaExpr) {
        match expr {
            Expr::Val(val) => (vec![], SsaExpr::Const(val.value())),
            Expr::Var(ident) => (vec![], SsaExpr::Var(ident.to_string())),
            Expr::Call {
                spread,
                f,
                mut args,
            } => {
                if args.iter().any(|a| matches!(a, Expr::Call { .. })) {
                    let mut statements = vec![];
                    let mut new_args = vec![];
                    for arg in args {
                        match arg {
                            expr @ (Expr::Var(_) | Expr::Val(_)) => new_args.push(expr),
                            arg @ Expr::Call { .. } => {
                                let (s, expr) = self.normalize_tac_statmenets_from_ast_expr(arg);
                                statements.extend(s);
                                let next_var = self.next_var();
                                new_args.push(Expr::Var(next_var.clone()));
                                statements.push(TacStatement::Assignment {
                                    var: next_var.to_string(),
                                    expr,
                                });
                            }
                        }
                    }

                    (
                        statements,
                        self.normalize_tac_statmenets_from_ast_expr(Expr::Call {
                            spread,
                            f,
                            args: new_args,
                        })
                        .1,
                    )
                } else {
                    match f.inner.clone() {
                        BuiltinOrDef::Builtin(builtin) => {
                            let e = SsaExpr::Builtin(match builtin {
                                Builtin::Add => BuiltinCall::Add(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Sub => BuiltinCall::Sub(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Mul => BuiltinCall::Mul(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Div => BuiltinCall::Div(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Exp => BuiltinCall::Exp(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Mod => BuiltinCall::Mod(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Eq => BuiltinCall::Eq(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Lt => BuiltinCall::Lt(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Gt => BuiltinCall::Gt(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Shl => BuiltinCall::Shl(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Shr => BuiltinCall::Shr(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Or => BuiltinCall::Or(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Xor => BuiltinCall::Xor(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::And => BuiltinCall::And(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Not => {
                                    BuiltinCall::Not(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Neg => {
                                    BuiltinCall::Neg(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread1 => {
                                    BuiltinCall::Dread1(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread2 => {
                                    BuiltinCall::Dread2(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread3 => {
                                    BuiltinCall::Dread3(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread4 => {
                                    BuiltinCall::Dread4(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread5 => {
                                    BuiltinCall::Dread5(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread6 => {
                                    BuiltinCall::Dread6(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread7 => {
                                    BuiltinCall::Dread7(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dread8 => {
                                    BuiltinCall::Dread8(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Dlen => BuiltinCall::Dlen(),
                                Builtin::Read1 => {
                                    BuiltinCall::Read1(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read2 => {
                                    BuiltinCall::Read2(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read3 => {
                                    BuiltinCall::Read3(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read4 => {
                                    BuiltinCall::Read4(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read5 => {
                                    BuiltinCall::Read5(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read6 => {
                                    BuiltinCall::Read6(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read7 => {
                                    BuiltinCall::Read7(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Read8 => {
                                    BuiltinCall::Read8(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Alloc => {
                                    BuiltinCall::Alloc(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                                Builtin::Write1 => BuiltinCall::Write1(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write2 => BuiltinCall::Write2(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write3 => BuiltinCall::Write3(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write4 => BuiltinCall::Write4(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write5 => BuiltinCall::Write5(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write6 => BuiltinCall::Write6(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write7 => BuiltinCall::Write7(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Write8 => BuiltinCall::Write8(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Dcopy => BuiltinCall::Dcopy(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Exit => BuiltinCall::Exit(
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                    Operand::from_ast_expr(args.pop().unwrap()),
                                ),
                                Builtin::Trap => {
                                    BuiltinCall::Trap(Operand::from_ast_expr(args.pop().unwrap()))
                                }
                            });

                            (vec![], e)
                        }
                        BuiltinOrDef::Def(ident) => {
                            bug!("not normalized: {}", Expr::Call { spread, f, args })
                        }
                    }
                }
            }
        }
    }

    fn next_var(&mut self) -> Ident<'static> {
        let id = self.counter;
        self.counter += 1;
        Ident::new(format!("t{id}"))
    }

    // fn normalize_expr(&mut self, expr: Expr<'a>) -> (Vec<Statement<'a>>,
    // Expr<'a>) {     match expr {
    //         Expr::Val(val) => (vec![], Expr::Val(val)),
    //         Expr::Var(var) => (vec![], Expr::Var(var)),
    //         Expr::Call { spread, f, args } => {
    //             if args.iter().any(|a| matches!(a, Expr::Call { .. })) {
    //                 let mut statements = vec![];
    //                 // let mut last_var = None;
    //                 let args_len = args.len();
    //                 let mut new_args = vec![];
    //                 for (idx, arg) in args.into_iter().enumerate() {
    //                     match arg {
    //                         expr @ (Expr::Var(_) | Expr::Val(_)) =>
    // new_args.push(expr),                         arg @ Expr::Call { .. } => {
    //                             let (s, expr) = self.normalize_expr(arg);
    //                             statements.extend(s);
    //                             let next_var = self.next_var();
    //                             new_args.push(Expr::Var(next_var.clone()));
    //                             statements.push(Statement::Assignment(Assignment
    // {                                 vars: vec![next_var],
    //                                 expr,
    //                             }));
    //                         }
    //                     }
    //                 }
    //                 (
    //                     statements,
    //                     Expr::Call {
    //                         spread,
    //                         f,
    //                         args: new_args,
    //                     },
    //                 )
    //             } else {
    //                 (vec![], Expr::Call { spread, f, args })
    //             }
    //         }
    //     }
    // }

    // fn normalize_if<'a>(
    //     &mut self,
    //     check_ctx: &CheckCtx<'a>,
    //     If { cond, block, else_ }: If<'a>,
    // ) -> (Vec<Statement<'a>>, If<'a>) {
    //     let mut new_block = vec![];
    //     let (statements, cond_expr) = self.normalize_expr(cond);
    //     new_block.extend(statements);
    //     let if_ = If {
    //         cond: cond_expr,
    //         block: self.run(check_ctx, block),
    //         // TODO: Run on else blocks
    //         else_: match else_ {
    //             Some(Else::ElseIf { if_ }) => {
    //                 let (statements, new_if) = self.normalize_if(check_ctx,
    // if_.inner);                 new_block.extend(statements);
    //                 Some(Else::ElseIf {
    //                     if_: Box::new(Spanned {
    //                         inner: new_if,
    //                         span: if_.span,
    //                     }),
    //                 })
    //             }
    //             Some(Else::Tail { block }) => Some(Else::Tail {
    //                 block: self.run(check_ctx, block),
    //             }),
    //             None => None,
    //         },
    //     };

    //     (new_block, if_)
    // }
}

fn print_tac(blocks: &IndexMap<String, BasicBlock>) -> String {
    blocks
        .iter()
        .map(|(label, block)| {
            format!(
                "{label}({}) -> {}\n",
                block
                    .args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                block
                    .rets
                    .iter()
                    .map(|ret| ret.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ) + &block
                .body
                .iter()
                .map(|statement| format!("    {statement}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn init() {
    let _ = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();
}

#[test]
fn compile_expr() {
    init();

    let raw = "
        var <- 2
        var2 <- 10
        if 1 {
            var <- add(var, var2)
        } else if eq(add(var2, var), 1) {
            trap(1)
        }
        ";

    let ast = grammar().block.parse(raw).unwrap();

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    println!("{}", print_tac(&ctx.blocks));
}

#[test]
fn test() {
    init();

    let raw = "
        # def add_mul(a, b) -> o {
        #     o <- mul(a, add(a, b))
        # }

        three <- 3
        v <- mul(2, add(1, three))

        alloc(8)
        write8(0, v)
        exit(0, 8)
        ";

    let ast = grammar().block.parse(raw).unwrap();
    // let ast = Normalize::new().run(&CheckCtx::new(""), ast);

    let mut ctx = Ctx::new_root();

    ctx.compile(&ast).unwrap();

    dbg!(ctx.blocks);
}

// #[test]
// fn compile_if_else_if_branch() {
//     init();

//     let raw = "
//             var <- 2
//             var2 <- 10
//             if 1 {
//                 var <- add(var, var2)
//             } else {
//                 trap(1)
//             }
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, None);

//     assert_eq!(
//         vm.stack,
//         [
//             12, // var
//             10  // var2
//         ]
//     );
// }

// #[test]
// fn compile_if_else_else_branch() {
//     init();

//     let raw = "
//             var <- 2
//             var2 <- 10
//             if eq(2, sub(var, 1)) {
//                 trap(1)
//             } else {
//                 var <- add(var, var2)
//             }
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, None);

//     assert_eq!(
//         vm.stack,
//         [
//             12, // var
//             10  // var2
//         ]
//     );
// }

// #[test]
// fn compile_if_else_if() {
//     init();

//     let raw = "
//             var <- 2
//             var2 <- 10
//             if 0 {
//                 trap(1)
//             } else if 0 {
//                 trap(2)
//             } else {
//                 var <- add(var, var2)
//             }
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, None);

//     assert_eq!(
//         vm.stack,
//         [
//             12, // var
//             10  // var2
//         ]
//     );
// }

// #[test]
// fn compile_def_single_arg() {
//     init();

//     let raw = "
//             def square(i) -> o {
//                 o <- mul(i, i)
//             }

//             five <- add(1, 4)
//             v <- square(five)

//             u <- add(1, v)

//             alloc(16)
//             write8(0, v)
//             write8(8, u)
//             exit(0, 16)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(
//         res,
//         Some(
//             [25_u64.to_be_bytes(), 26_u64.to_be_bytes()]
//                 .as_flattened()
//                 .to_vec()
//         )
//     );
// }

// #[test]
// fn compile_def_multiple_args() {
//     init();

//     let raw = "
//             def add_mul(a, b) -> o {
//                 o <- mul(a, add(a, b))
//             }

//             three <- 3
//             v <- add_mul(three, 5)

//             alloc(8)
//             write8(0, v)
//             exit(0, 8)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(24_u64.to_be_bytes().to_vec()));
// }

// #[test]
// fn fib_recursive() {
//     init();

//     let raw = "
//             def fib(n) -> m {
//                 if eq(n, 0) {
//                     m <- 0
//                 }

//                 if eq(n, 1) {
//                     m <- 1
//                 }

//                 if gt(n, 1) {
//                     m <- add(fib(sub(n, 1)), fib(sub(n, 2)))
//                 }
//             }

//             res <- fib(10)

//             alloc(8)
//             write8(0, res)
//             exit(0, 8)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(55_u64.to_be_bytes().to_vec()));
// }

// #[test]
// fn compile_def_shadowing() {
//     init();

//     let raw = "
//             def digit_to_place(digit, idx) -> n {
//               n <- mul(digit, exp(10, sub(dlen(), add(idx, 1))))
//             }

//             if eq(dlen(), 0) {
//               trap(1)
//             }

//             n <- 0
//             idx <- 0

//             loop :a {
//               if eq(dlen(), idx) {
//                 break :a
//               }

//               ascii_digit <- dread1(idx)

//               if lt(ascii_digit, 0x30) {
//                 trap(2)
//               }

//               if gt(ascii_digit, 0x39) {
//                 trap(3)
//               }

//               digit <- sub(ascii_digit, 0x30)
//               n <- add(n, digit_to_place(digit, idx))
//               idx <- add(idx, 1)
//             }

//             alloc(8)
//             write8(0, n)
//             exit(0, 8)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, b"123".to_vec());

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(123_u64.to_be_bytes().to_vec()));
// }

// #[test]
// fn multiple_return_values() {
//     init();

//     let raw = "
//             def many(a) -> b, c, d, e, f {
//                 b <- add(a, 1)
//                 c <- add(a, 2)
//                 d <- add(a, 3)
//                 e <- add(a, 4)
//                 f <- add(a, 5)
//             }

//             a <- 100

//             b, c, d, e, f <- many(a)

//             alloc(6)
//             write1(0, a)
//             write1(1, b)
//             write1(2, c)
//             write1(3, d)
//             write1(4, e)
//             write1(5, f)
//             exit(0, 6)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(vec![100, 101, 102, 103, 104, 105]));

//     assert_eq!(vm.stack, [100, 101, 102, 103, 104, 105]);
// }

// #[test]
// fn multiple_return_values_update_and_init() {
//     init();

//     let raw = "
//             def foo(a, b) -> c, d, e {
//                 c <- b
//                 d <- a
//                 e <- 0x22
//             }

//             a <- 0x11
//             c <- 0x33

//             a, b, c <- foo(a, c)

//             alloc(3)
//             write1(0, a)
//             write1(1, b)
//             write1(2, c)
//             exit(0, 3)
//         ";

//     // # 0x33, 0x11, 0x22

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(
//         res,
//         Some(vec![
//             0x33, // a
//             0x11, // b
//             0x22, // c
//         ])
//     );

//     // a and c are pushed to the stack, then b when it is first set in the
// multi     // assignment along with a and c being updated
//     assert_eq!(
//         vm.stack,
//         [
//             0x33, // a
//             0x22, // c
//             0x11, // b
//         ]
//     );
// }

// #[test]
// fn multiple_return_values_as_args() {
//     init();

//     let raw = "
//             def foo(a) -> c, d {
//                 c <- mul(10, a)
//                 d <- mul(2, a)
//             }

//             a <- sub(...foo(4))

//             alloc(1)
//             write1(0, a)
//             exit(0, 1)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(vec![(10 * 4) - (2 * 4)]));

//     assert_eq!(vm.stack, [(10 * 4) - (2 * 4)]);
// }

// #[test]
// fn multiple_return_values_as_args_complex() {
//     init();

//     let raw = "
//             def foo(a) -> c, d {
//                 c <- mul(10, a)
//                 d <- mul(2, a)
//             }

//             def bar(a, b, c) -> d {
//                 d <- mul(a, add(b, c))
//             }

//             def baz(a) -> d {
//                 d <- add(a, 1)
//             }

//             a <- 1
//             res <- bar(...foo(4), a)

//             alloc(3)
//             write1(0, a)
//             write2(1, res)
//             exit(0, 3)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(
//         res,
//         Some(
//             [[1_u8].as_slice(), 360_u16.to_be_bytes().as_slice()]
//                 .into_iter()
//                 .flatten()
//                 .copied()
//                 .collect::<Vec<_>>()
//         )
//     );
// }

// #[test]
// fn multiple_return_swap_params() {
//     init();

//     let raw = "
//             def swap(a_, b_) -> c, d {
//                 d <- a_
//                 c <- b_
//             }

//             a <- 0xaa
//             b <- 0xbb

//             a, b <- swap(a, b)

//             alloc(2)
//             write1(0, a)
//             write1(1, b)
//             exit(0, 2)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(vec![0xbb, 0xaa]));

//     assert_eq!(vm.stack, [0xbb, 0xaa]);
// }

// #[test]
// fn compile_loop() {
//     init();

//     let raw = "
//             counter <- 0x00

//             loop :a {
//               counter <- add(counter, 1)
//               if eq(counter, 10) {
//                 break :a
//               }
//             }

//             alloc(1)
//             write1(0, counter)
//             exit(0, 1)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(vec![10]));
// }

// #[test]
// fn compile_loop_shadow_label() {
//     init();

//     let raw = "
//             counter <- 0x00

//             loop :a {
//               loop :a {
//                 counter <- add(counter, 1)
//                 if eq(counter, 10) {
//                   break :a
//                 }
//               }
//               break :a
//             }

//             alloc(1)
//             write1(0, counter)
//             exit(0, 1)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, vec![]);

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(vec![10]));
// }

// #[test]
// fn compile_atoi() {
//     // # n += digit * (10 ** (dlen() - (idx + 1)))
//     let raw = "
//             if eq(dlen(), 0) {
//               trap(1)
//             }

//             n <- 0
//             idx <- 0

//             loop :a {
//               if eq(dlen(), idx) {
//                 break :a
//               }

//               ascii_digit <- dread1(idx)

//               if lt(ascii_digit, 0x30) {
//                 trap(2)
//               }

//               if gt(ascii_digit, 0x39) {
//                 trap(3)
//               }

//               digit <- sub(ascii_digit, 0x30)
//               n <- add(n, mul(digit, exp(10, sub(dlen(), add(idx, 1)))))
//               idx <- add(idx, 1)
//             }

//             alloc(8)
//             write8(0, n)
//             exit(0, 8)
//             ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, b"1234567".into());

//     let res = vm.run().unwrap();

//     assert_eq!(res.unwrap(), 1234567_u64.to_be_bytes());
// }

// #[test]
// fn compile_aoc_2025_1() {
//     init();

//     let raw = "
//         if eq(dlen(), 0) {
//           trap(1)
//         }

//         dial <- 100050
//         total <- 0
//         n <- 0
//         idx <- 0
//         is_right <- 0

//         loop :a {
//           if eq(dlen(), idx) {
//             break :a
//           }

//           ascii_digit <- dread1(idx)

//           if eq(ascii_digit, 76) {
//             is_right <- 0

//             idx <- add(idx, 1)
//             n <- 0
//             continue :a
//           } else if eq(ascii_digit, 82) {
//             is_right <- 1

//             idx <- add(idx, 1)
//             n <- 0
//             continue :a
//           } else if eq(ascii_digit, 10) {
//             idx <- add(idx, 1)
//             if is_right {
//               dial <- add(dial, n)
//               if eq(0, mod(dial, 100)) {
//                 total <- add(total, 1)
//               }

//               continue :a
//             }

//             dial <- sub(dial, n)
//             if eq(0, mod(dial, 100)) {
//               total <- add(total, 1)
//             }

//             continue :a
//           }

//           if lt(ascii_digit, 0x30) {
//             trap(2)
//           }

//           if gt(ascii_digit, 0x39) {
//             trap(3)
//           }

//           digit <- sub(ascii_digit, 0x30)
//           n <- mul(n, 10)
//           n <- add(n, digit)
//           idx <- add(idx, 1)
//         }

//         alloc(8)
//         write8(0, total)
//         exit(0, 8)
//         ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     println!("{obj}");
//     // dbg!(&obj);

//     let asm = obj.assemble();

//     let mut vm = Vm::new(
//         asm,
//         b"L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
// "
//         .into(),
//     );

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(3_u64.to_be_bytes().to_vec()))
// }

// #[test]
// fn drop_vars_in_if_block() {
//     init();

//     let raw = "
// x <- 1
// y <- 0
// t <- 0
// loop :a {
//   if lt(t, 24) {
//     Y <- mod(add(mul(2, x), mul(3, y)), 5)
//     t <- add(t, 1)
//   } else {
//     break :a
//   }
// }
// alloc(8)
// write8(0, t)
// exit(0, 8)
// ";

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     println!("{obj}");

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, b"".into());

//     let res = vm.run().unwrap();

//     assert_eq!(res, Some(24_u64.to_be_bytes().to_vec()))
// }

// #[test]
// fn drop_vars_in_def_body() {
//     init();

//     let raw = r#"
// def f(at) -> u {
//   i <- 7
// }

// f(0)
//         "#;

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, b"".into());

//     let res = vm.run().unwrap();

//     assert_eq!(res, None)
// }

// #[test]
// fn stack_depth_after_call_is_correct() {
//     init();

//     let raw = r#"
// def inner(a, b, inner_at, value, c, d) {
//   write8(inner_at, value)
// }

// def outer(a, b, outer_at, value, c, d) -> n, m {
//   inner(a, b, outer_at, value, c, d)
//   n <- 0xaa
//   m <- 0xbb
//   # inner(y, inner_at, z)
// }

// alloc(8)
// n, m <- outer(0xa, 0xb, 0, 0xFFF, 0xc, 0xd)
// trap(n)
//         "#;

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     ctx.compile(&ast).unwrap();

//     let obj = ctx.into_object();

//     let asm = obj.assemble();

//     let mut vm = Vm::new(asm, b"".into());

//     let err = vm.run().unwrap_err();

//     assert_eq!(err, Error::Trap(0xaa));
// }

// #[test]
// fn outer_def_cannot_refer_to_called_def_arg() {
//     init();

//     let raw = r#"
// def inner(inner_at) {
//   write8(inner_at, 0x0)
// }

// def outer(outer_at) -> n, m {
//   inner(inner_at)
// }

// alloc(8)
// outer(1)
//         "#;

//     let ast = grammar().block.parse(raw).unwrap();

//     let mut ctx = Ctx::new_root();

//     let err = ctx.compile(&ast).unwrap_err();

//     assert_eq!(
//         err,
//         CompileError::VarNotFound {
//             var: "inner_at".to_owned()
//         }
//     );
// }
