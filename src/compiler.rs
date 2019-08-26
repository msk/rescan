use crate::database::Database;
use crate::nfagraph::{make_nfa_builder, Ng, NgHolder};
use crate::parser::{make_glushkov_build_state, parse, shortcut_literal, Component, ParseMode};
use crate::rose::RoseEngine;
use crate::ue2common::ReportId;
use crate::{CompileError, ErrorKind};

pub(crate) struct ParsedExpression {
    pub(crate) component: Box<dyn Component>,
}

impl ParsedExpression {
    fn new(_index: u32, expression: &str) -> Result<Self, CompileError> {
        let mut mode = ParseMode::default();

        let component = parse(expression, &mut mode)?;
        Ok(ParsedExpression { component })
    }
}

pub(crate) struct BuiltExpression {
    pub(crate) g: NgHolder,
}

pub(crate) fn add_expression(
    ng: &mut Ng,
    index: u32,
    expression: &str,
    _id: ReportId,
) -> Result<(), CompileError> {
    let cc = &ng.cc;

    if expression.len() > cc.grey.limit_pattern_length {
        return Err(CompileError::new(
            ErrorKind::Other,
            "Pattern length exceeds limit.",
        ));
    }

    let pe = ParsedExpression::new(index, expression)?;

    // If this expression is a literal, we can feed it directly to Rose rather
    // than building the NFA graph.
    if shortcut_literal(ng, &pe) {
        return Ok(());
    }

    let build_expr = build_graph(&pe);

    ng.add_graph(build_expr.g);

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

pub(crate) fn build_graph(_pe: &ParsedExpression) -> BuiltExpression {
    let mut builder = make_nfa_builder();

    let _bs = make_glushkov_build_state(&mut builder);

    builder.get_graph()
}
