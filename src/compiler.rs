use crate::parser::{parse, ParseMode};
use crate::ue2common::ReportId;
use crate::util::compile_error::CompileError;

struct ParsedExpression {}

impl ParsedExpression {
    fn new(_index: u32, expression: &str) -> Result<Self, CompileError> {
        let mut mode = ParseMode::default();

        parse(expression, &mut mode)?;
        Ok(ParsedExpression {})
    }
}

pub(crate) fn add_expression(
    index: u32,
    expression: &str,
    _id: ReportId,
) -> Result<(), CompileError> {
    let _pe = ParsedExpression::new(index, expression)?;
    Ok(())
}
