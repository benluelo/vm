use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, iter,
};

use chumsky::span::Spanned;
use indexmap::IndexMap;
use petgraph::graph::DiGraph;
use tracing::{info, info_span, instrument, trace};

use crate::{
    assembler::{AsmOp, Object},
    mir::ast::{
        Assignment, Block, Break, Builtin, BuiltinOrDef, Continue, Def, Else, Expr, Ident, If,
        Label, Loop, Statement,
    },
};

pub mod parse;
pub mod pass;

pub mod ast;
// pub mod cfg;

#[cfg(test)]
mod tests;

type Section<'a> = IndexMap<String, Vec<AsmOp<'a>>>;

#[derive(Debug)]
pub struct Ctx<'a> {
    cfg: DiGraph<Node<'a>, Edge>,
    next_scope_label_id: LabelId,
    salt_id_counter: Id,
    prefix: String,
    sections: Section<'a>,
    fns: IndexMap<String, Section<'a>>,
    stack_depth: usize,
    scopes: Vec<Scope<'a>>,
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
            cfg: DiGraph::new(),
            next_scope_label_id: first_scope_label_id.increment(),
            salt_id_counter: Id::new(),
            prefix: prefix.to_owned(),
            sections: [(first_section_id, vec![])].into_iter().collect(),
            stack_depth: 0,
            fns: Default::default(),
            scopes: [scope].into_iter().collect(),
        }
    }

    pub fn into_object(self) -> Object<'a> {
        let root_label = self
            .scopes
            .first()
            .unwrap()
            .label
            .as_label()
            .unwrap()
            .to_owned()
            .to_string()
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

    fn get_var(&self, var: Ident<'a>) -> Option<usize> {
        self.scopes
            .iter()
            .find_map(|s| s.vars.iter().find_map(|(v, i)| v.eq(&var).then_some(*i)))
    }

    fn init_var<'b>(&'b mut self, var: Ident<'a>) -> usize {
        self.init_var_with_depth_offset(var, 0)
    }

    fn init_var_with_depth_offset(&mut self, var: Ident<'a>, depth: isize) -> usize {
        info!("PUSHING VAR {var} @ {depth}");
        let var_idx = self.stack_depth.strict_add_signed(depth);
        self.scopes
            .last_mut()
            .expect("no scopes?")
            .vars
            .insert(var, var_idx);
        var_idx
    }

    fn get_def(&self, def: Ident<'a>) -> Option<&(Def<'a>, String)> {
        self.scopes
            .iter()
            .find_map(|s| s.defs.iter().find_map(|(d, i)| d.eq(&def).then_some(i)))
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
                    }
                    Statement::Loop(Loop { label, block }) => {
                        trace!("loop");
                        let identified_label = ctx.new_label(*label);
                        let loop_start_label = ctx.loop_start_label(&identified_label);
                        let loop_end_label = ctx.loop_end_label(&identified_label);
                        ctx.push_section(&loop_start_label);
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
                            AsmOp::PUSHL(loop_start_label.into()),
                            AsmOp::JUMP,
                        ]);
                        ctx.push_section(&loop_end_label);
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

                        ctx.current_section()
                            .extend_from_slice(&[AsmOp::PUSHL(loop_end_label.into()), AsmOp::JUMP]);
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
                            AsmOp::PUSHL(loop_start_label.into()),
                            AsmOp::JUMP,
                        ]);
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

                            ctx.push_section(&format!(
                                "{}:if_block_[{}]",
                                ctx.prefix,
                                block.span()
                            ));

                            ctx.push_scope("if block".to_owned(), ScopeLabel::None);
                            go(ctx, depth + 1, &block)?;
                            ctx.pop_scope(ScopeLabel::None, true)?;

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
                    Statement::Assignment(Assignment { vars, expr }) => {
                        let arity = ctx.expr_arity(0, expr, true)?;
                        assert_eq!(vars.len(), arity);

                        // def f() -> a, b, c {}
                        // d, e, f <- f()
                        // # pushed to the stack in this order:
                        // # [c, b, a]

                        // if any vars on the lhs are updates, then init any newly declared vars
                        // first before evaluating the rhs
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
                        ctx.compile_expr(expr)?;

                        for (i, var) in vars.iter().rev().enumerate() {
                            match ctx.get_var(*var) {
                                // var declaration, initial value was already pushed to the stack
                                // above when evaluating the rhs
                                // expression, so just store the variable's
                                // stack position
                                None => {
                                    trace!("var decl '{var}' (i: {i})");
                                    let idx =
                                        ctx.init_var_with_depth_offset(*var, -((i + 1) as isize));
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
                    }
                    Statement::Def(def) => {
                        info_span!("def", name = %def.ident).in_scope(|| -> CompileResult {
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

        let root = self.cfg.add_node(Node::Root);

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
                let BuiltinOrDef::Def(def) = def.inner else {
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
                    let Some(idx) = ctx.get_var(*var) else {
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

                    match f.inner {
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
}

#[derive(Debug)]
enum Node<'a> {
    Root,
    CallEntry,
    CallExit,
    Expr(Expr<'a>),
    Assignment(Assignment<'a>),
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Root => f.write_str("root"),
            Node::Expr(expr) => f.write_fmt(format_args!("{expr}")),
            Node::Assignment(assignment) => f.write_fmt(format_args!("{assignment}")),
            _ => todo!(),
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Edge::None => f.write_str(""),
            Edge::Break => f.write_str("break"),
            Edge::Continue => f.write_str("continue"),
            Edge::IfTrue => f.write_str("if true"),
            Edge::IfFalse => f.write_str("if false"),
        }
    }
}

#[derive(Debug)]
pub struct CheckCtx<'a> {
    prefix: String,
    next_scope_label_id: LabelId,
    scopes: Vec<CheckScope<'a>>,
}

#[derive(Debug)]
pub struct CheckScope<'a> {
    #[expect(dead_code)]
    tag: String,
    label: ScopeLabel<'a>,
    vars: BTreeSet<Ident<'a>>,
    /// fn name -> label
    defs: BTreeMap<Ident<'a>, Def<'a>>,
}

impl<'a> CheckCtx<'a> {
    fn push_scope(&mut self, tag: String, label: ScopeLabel<'a>) {
        trace!("pushing scope {label} ({tag})",);
        self.scopes.push(CheckScope {
            tag,
            label,
            vars: Default::default(),
            defs: Default::default(),
        });
    }

    fn pop_scope(&mut self, label: ScopeLabel<'a>) -> CompileResult {
        trace!("pop_scope {label}",);

        loop {
            match self.scopes.pop() {
                Some(scope) => {
                    trace!("popping scope {}", scope.label);
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

    fn cleanup_scopes_to_label(&mut self, label: Label<'a>) -> CompileResult<()> {
        trace!("scope_cleanup_asm {}", label);

        for scope in self.scopes.iter().rev() {
            tracing::info!("appending drop asm for scope {}", scope.label);

            if scope.label.matches_label(&label) {
                return Ok(());
            }
        }

        Err(CompileError::LabelNotFound {
            label: label.to_string(),
        })
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
                let BuiltinOrDef::Def(def) = def.inner else {
                    bug!("attempted to call builtin {} as a def", def.inner)
                };

                let arity = self
                    .get_def(&def)
                    .ok_or_else(|| CompileError::DefNotFound {
                        def: def.to_string(),
                    })?
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

    fn get_def(&self, def: &Ident<'a>) -> Option<&Def<'a>> {
        self.scopes
            .iter()
            .find_map(|s| s.defs.iter().find_map(|(d, i)| d.eq(def).then_some(i)))
    }

    pub fn check<'b>(&mut self, block: &'b Block<'a>) -> CompileResult
    where
        'a: 'b,
    {
        fn go<'a: 'b, 'b>(
            ctx: &mut CheckCtx<'a>,
            depth: usize,
            block: &'b Block<'a>,
        ) -> CompileResult {
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
                        ctx.expr_arity(0, expr, false)?;
                        ctx.check_expr(expr)?;
                    }
                    Statement::Loop(Loop { label, block }) => {
                        trace!("loop");
                        let identified_label = ctx.new_label(*label);
                        let scope_label = ScopeLabel::Label(identified_label);
                        ctx.push_scope(format!("loop {label}"), scope_label.clone());
                        go(ctx, depth + 1, block)?;
                        // append scope cleanup code just before jumping back to the beginning of
                        // the loop
                        ctx.cleanup_scopes_to_label(*label)?;
                        // exit scope
                        ctx.pop_scope(scope_label)?;
                    }
                    Statement::Break(Break(label)) => {
                        trace!("break");

                        ctx.cleanup_scopes_to_label(*label)?;

                        trace!("cleaned up scope '{label}'");
                    }
                    Statement::Continue(Continue(label)) => {
                        trace!("continue");

                        ctx.cleanup_scopes_to_label(*label)?;
                    }
                    Statement::If(if_) => {
                        fn go_if<'a>(
                            ctx: &mut CheckCtx<'a>,
                            If { cond, block, else_ }: If<'a>,
                            depth: usize,
                        ) -> CompileResult {
                            // evaluate condition expression
                            ctx.check_expr(&cond)?;

                            ctx.push_scope("if block".to_owned(), ScopeLabel::None);
                            go(ctx, depth + 1, &block)?;
                            ctx.pop_scope(ScopeLabel::None)?;

                            if let Some(else_) = else_ {
                                match else_ {
                                    Else::ElseIf { if_ } => {
                                        trace!("else if");
                                        go_if(ctx, if_.inner, depth + 1)?
                                    }
                                    Else::Tail { block } => {
                                        trace!("else");
                                        go(ctx, depth + 1, &block)?;
                                    }
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
                        for var in vars.iter().rev() {
                            if !ctx.has_var(*var) {
                                trace!("var decl '{var}'");
                                ctx.init_var(*var);
                            }
                        }

                        // evaluate the expression
                        ctx.check_expr(expr)?;
                    }
                    Statement::Def(def) => {
                        info_span!("def", name = %def.ident).in_scope(|| -> CompileResult {
                            // // args.len() + 1 for return pointer
                            // assert!(ctx.stack_depth > def.args.len());

                            let def_label = format!("{}:def_{}_{depth}_{i}", ctx.prefix, def.ident);
                            // this function is callable in this scope
                            ctx.scopes
                                .last_mut()
                                .unwrap()
                                .defs
                                .insert(def.ident, def.clone());

                            let mut def_ctx = CheckCtx::new(&format!("{}/{def_label}", ctx.prefix));

                            // calling convention is [...args, @caller_ptr, ...rets]
                            // args will be popped before returning
                            // output is [...rets]
                            // therefore, before calling the final JUMP op, the stack must be
                            // [...rets, @caller_ptr]

                            // args are provided by the caller, init them in the new ctx
                            for arg in &def.args {
                                trace!("arg '{arg}'");
                                def_ctx.init_var(*arg);
                            }

                            // account for @caller_ptr, also provided by the caller
                            // NOTE: The return pointer is pushed at the callsite by CALL
                            trace!("@caller_ptr");

                            // new ctx values for this fn call

                            // init return values
                            for ret in def.rets.iter().rev() {
                                trace!("ret '{ret}'");
                                def_ctx.init_var(*ret);
                            }

                            // functions can access other functions visible in this scope
                            for (def_name, label) in ctx.scopes.iter().flat_map(|s| &s.defs) {
                                def_ctx
                                    .scopes
                                    .last_mut()
                                    .unwrap()
                                    .defs
                                    .insert(*def_name, label.clone());
                            }

                            def_ctx
                                .push_scope(format!("def '{}' body", def.ident), ScopeLabel::None);
                            // compile the fn body
                            go(&mut def_ctx, depth + 1, &def.body)?;
                            def_ctx.pop_scope(ScopeLabel::None)?;

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

            Ok(())
        }

        go(self, 0, block)
    }

    fn has_var(&self, var: Ident<'a>) -> bool {
        self.scopes
            .iter()
            .any(|s| s.vars.iter().any(|v| v.eq(&var)))
    }

    fn init_var<'b>(&'b mut self, var: Ident<'a>) {
        self.scopes.last_mut().expect("no scopes?").vars.insert(var);
    }

    fn check_expr(&mut self, expr: &Expr<'a>) -> CompileResult<()> {
        #[instrument(level = "TRACE", skip_all, fields(%expr))]
        fn go<'a>(ctx: &mut CheckCtx<'a>, depth: usize, expr: &Expr<'a>) -> CompileResult {
            trace!("evaluating: {expr}");

            match expr {
                Expr::Val(val) => {
                    trace!("val {val:#x}");
                }
                Expr::Var(var) => {
                    if !ctx.has_var(*var) {
                        return Err(CompileError::VarNotFound {
                            var: var.to_string(),
                        });
                    };
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
                        ctx: &mut CheckCtx<'a>,
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

                    match f.inner {
                        BuiltinOrDef::Builtin(Builtin::Add) => {
                            ensure_arity_and_eval_args(ctx, depth, "add", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Mul) => {
                            ensure_arity_and_eval_args(ctx, depth, "mul", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Sub) => {
                            ensure_arity_and_eval_args(ctx, depth, "sub", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Div) => {
                            ensure_arity_and_eval_args(ctx, depth, "div", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Exp) => {
                            ensure_arity_and_eval_args(ctx, depth, "exp", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Mod) => {
                            ensure_arity_and_eval_args(ctx, depth, "mod", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Eq) => {
                            ensure_arity_and_eval_args(ctx, depth, "eq", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Lt) => {
                            ensure_arity_and_eval_args(ctx, depth, "lt", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Gt) => {
                            ensure_arity_and_eval_args(ctx, depth, "gt", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Shl) => {
                            ensure_arity_and_eval_args(ctx, depth, "shl", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Shr) => {
                            ensure_arity_and_eval_args(ctx, depth, "shr", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Or) => {
                            ensure_arity_and_eval_args(ctx, depth, "or", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Xor) => {
                            ensure_arity_and_eval_args(ctx, depth, "xor", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::And) => {
                            ensure_arity_and_eval_args(ctx, depth, "and", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Not) => {
                            ensure_arity_and_eval_args(ctx, depth, "not", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Neg) => {
                            ensure_arity_and_eval_args(ctx, depth, "neg", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Alloc) => {
                            ensure_arity_and_eval_args(ctx, depth, "alloc", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Write1) => {
                            ensure_arity_and_eval_args(ctx, depth, "write1", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Write2) => {
                            ensure_arity_and_eval_args(ctx, depth, "write2", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Write8) => {
                            ensure_arity_and_eval_args(ctx, depth, "write8", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Read1) => {
                            ensure_arity_and_eval_args(ctx, depth, "read1", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Read8) => {
                            ensure_arity_and_eval_args(ctx, depth, "read8", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Dread1) => {
                            ensure_arity_and_eval_args(ctx, depth, "dread1", 1, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Dcopy) => {
                            ensure_arity_and_eval_args(ctx, depth, "dcopy", 3, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Dlen) => {
                            ensure_arity_and_eval_args(ctx, depth, "dlen", 0, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Exit) => {
                            ensure_arity_and_eval_args(ctx, depth, "exit", 2, exprs)?;
                        }
                        BuiltinOrDef::Builtin(Builtin::Trap) => {
                            ensure_arity_and_eval_args(ctx, depth, "trap", 1, exprs)?;
                        }
                        BuiltinOrDef::Def(f) => {
                            trace!("call '{f}'");

                            let def = ctx.get_def(&f).expect("def not found").clone();

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

                            // all args are dropped from the stack
                            ctx.pop_scope(ScopeLabel::None)?;
                        }
                        _ => todo!(),
                    }
                }
            }

            Ok(())
        }

        go(self, 0, expr)
    }

    pub fn new(prefix: &str) -> Self {
        let first_scope_label_id = LabelId::new();
        let scope = CheckScope {
            tag: "root".to_owned(),
            label: ScopeLabel::Label(IdentifiedLabel::new(
                Label::new(
                    // TODO: Figure out a better way to do this
                    format!("$ROOT/{prefix}").leak(),
                ),
                first_scope_label_id,
            )),
            vars: Default::default(),
            defs: Default::default(),
        };
        Self {
            prefix: prefix.to_owned(),
            next_scope_label_id: first_scope_label_id.increment(),
            scopes: vec![scope],
        }
    }

    fn new_label(&mut self, label: Label<'a>) -> IdentifiedLabel<'a> {
        self.next_scope_label_id = self.next_scope_label_id.increment();
        IdentifiedLabel::new(label, self.next_scope_label_id)
    }
}

pub enum Scope2<'a> {
    Loop {
        label: Label<'a>,
        locals: BTreeMap<Ident<'a>, usize>,
    },
    IfElse {
        locals: BTreeMap<Ident<'a>, usize>,
    },
    DefOuter {
        args: BTreeMap<Ident<'a>, usize>,
        rets: BTreeMap<Ident<'a>, usize>,
    },
    DefBody {
        locals: BTreeMap<Ident<'a>, usize>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct LabelId(u32);

impl LabelId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for LabelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentifiedLabel<'a> {
    label: Label<'a>,
    id: LabelId,
}

impl<'a> IdentifiedLabel<'a> {
    pub fn new(label: Label<'a>, id: LabelId) -> Self {
        Self { label, id }
    }
}

impl fmt::Display for IdentifiedLabel<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.label, self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeLabel<'a> {
    None,
    Label(IdentifiedLabel<'a>),
}

impl<'a> ScopeLabel<'a> {
    pub fn as_label(&self) -> Option<&IdentifiedLabel<'a>> {
        if let Self::Label(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn matches_scope_label(&self, other: &ScopeLabel<'_>) -> bool {
        match (self, other) {
            (ScopeLabel::None, _) => true,
            (ScopeLabel::Label(_), ScopeLabel::None) => false,
            (ScopeLabel::Label(this), ScopeLabel::Label(other)) => this == other,
        }
    }

    pub fn matches_label(&self, other: &Label<'_>) -> bool {
        match self {
            ScopeLabel::None => false,
            ScopeLabel::Label(il) => &il.label == other,
        }
    }
}

impl fmt::Display for ScopeLabel<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScopeLabel::None => write!(f, "<none>"),
            ScopeLabel::Label(il) => write!(f, "{il}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Id(u32);

impl Id {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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
