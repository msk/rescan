mod expression_info;

use crate::database::Database;
use crate::nfagraph::{make_nfa_builder, Ng, NgHolder};
use crate::parser::{
    make_glushkov_build_state, parse, prefilter_tree, shortcut_literal, Component, ParseMode,
};
use crate::rose::RoseEngine;
use crate::Flags;
use crate::{CompileError, ErrorKind};
use expression_info::ExpressionInfo;
use rescan_util::ReportId;

pub(crate) struct ParsedExpression {
    expr: ExpressionInfo,
    pub(crate) component: Component,
}

impl ParsedExpression {
    fn new(_index: usize, expression: &str, flags: Flags) -> Result<Self, CompileError> {
        let mut expr = ExpressionInfo {
            utf8: false,
            prefilter: flags.contains(Flags::PREFILTER),
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
    pub(crate) g: NgHolder,
}

pub(crate) fn add_expression(
    ng: &mut Ng,
    index: usize,
    expression: &str,
    flags: Flags,
    _id: ReportId,
) -> Result<(), CompileError> {
    let cc = &ng.cc;

    if expression.len() > cc.grey.limit_pattern_length {
        return Err(CompileError::new(
            ErrorKind::Other,
            "Pattern length exceeds limit.",
        ));
    }

    let mut pe = ParsedExpression::new(index, expression, flags)?;

    // Apply prefiltering transformations if desired.
    if pe.expr.prefilter {
        prefilter_tree(&pe.component);
    }

    // If this expression is a literal, we can feed it directly to Rose rather
    // than building the NFA graph.
    if shortcut_literal(ng, &pe) {
        return Ok(());
    }

    let build_expr = build_graph(&mut pe);

    ng.add_graph(&build_expr.g);

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
    let mut builder = make_nfa_builder();

    let mut bs = make_glushkov_build_state(&mut builder);

    // Map position IDs to characters/components.
    pe.component.note_positions(&mut bs);

    builder.get_graph()
}
