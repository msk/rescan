use crate::nfagraph::Ng;
use crate::parser::{parse, shortcut_literal, Component, ParseMode};
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

    Ok(())
}
