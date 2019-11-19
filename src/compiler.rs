mod expression_info;

pub(crate) use expression_info::ExpressionInfo;

use crate::database::Database;
use crate::nfagraph::{make_nfa_builder, Ng, NgHolder};
use crate::parser::{
    make_glushkov_build_state, parse, prefilter_tree, shortcut_literal, Component, ParseMode,
};
use crate::rose::RoseEngine;
use crate::util::ReportId;
use crate::{CompileError, ErrorKind};
use crate::{Flags, SomType};
use std::pin::Pin;

pub(crate) struct ParsedExpression {
    pub(crate) expr: ExpressionInfo,
    pub(crate) component: Component,
}

impl ParsedExpression {
    fn new(
        index: usize,
        expression: &str,
        flags: Flags,
        id: ReportId,
    ) -> Result<Self, CompileError> {
        let mut expr = ExpressionInfo {
            index,
            report: id,
            allow_vacuous: false,
            highlander: flags.contains(Flags::SINGLEMATCH),
            utf8: false,
            prefilter: flags.contains(Flags::PREFILTER),
            som: SomType::None,
            quiet: flags.contains(Flags::QUIET),
        };
        if flags.contains(Flags::QUIET | Flags::SOM_LEFTMOST) {
            return Err(CompileError::new(
                ErrorKind::Other,
                "QUIET is not supported in combination with SOM_LEFTMOST.",
            ));
        }
        let flags = flags & !Flags::QUIET;
        let mut mode = ParseMode::new(flags);

        let component = parse(expression, &mut mode)?;

        expr.utf8 = mode.utf8; // utf8 may be set by parse()

        Ok(Self { expr, component })
    }
}

pub(crate) struct BuiltExpression {
    pub(crate) expr: ExpressionInfo,
    pub(crate) g: NgHolder,
}

pub(crate) fn add_expression(
    ng: &mut Pin<Box<Ng>>,
    index: usize,
    expression: &str,
    flags: Flags,
    id: ReportId,
) -> Result<(), CompileError> {
    let cc = &ng.cc;

    if expression.len() > cc.grey.limit_pattern_length {
        return Err(CompileError::new(
            ErrorKind::Other,
            "Pattern length exceeds limit.",
        ));
    }

    let mut pe = ParsedExpression::new(index, expression, flags, id)?;

    // Apply prefiltering transformations if desired.
    if pe.expr.prefilter {
        prefilter_tree(&pe.component);
    }

    // If this expression is a literal, we can feed it directly to Rose rather
    // than building the NFA graph.
    if shortcut_literal(ng, &pe)? {
        return Ok(());
    }

    let build_expr = build_graph(&mut pe);

    unsafe {
        let inner: Pin<&mut Ng> = ng.as_mut();
        inner
            .get_unchecked_mut()
            .add_graph(&build_expr.expr, &build_expr.g);
    };

    Ok(())
}

fn generate_rose_engine(ng: &Ng) -> RoseEngine {
    ng.rose.build_rose()
}

fn db_create(rose: RoseEngine) -> Database {
    Database::new(rose)
}

pub(crate) fn build(ng: &Ng) -> Database {
    let rose = generate_rose_engine(ng);

    db_create(rose)
}

pub(crate) fn build_graph(pe: &mut ParsedExpression) -> BuiltExpression {
    let mut builder = make_nfa_builder(pe);

    let mut bs = make_glushkov_build_state(&mut builder);

    // Map position IDs to characters/components.
    pe.component.note_positions(&mut bs);

    builder.get_graph()
}
