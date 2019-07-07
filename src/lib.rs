mod compiler;
mod parser;
mod ue2common;
mod util;

use compiler::add_expression;
pub use ue2common::ReportId;
pub use util::compile_error::{CompileError, ErrorKind};

/// Compiles a regular expression.
pub fn compile(expression: &str) -> Result<(), CompileError> {
    compile_multi_int(&[expression], &[0])
}

/// Compiles multiple regular expressions.
pub fn compile_multi(expressions: &[&str], ids: &[ReportId]) -> Result<(), CompileError> {
    compile_multi_int(expressions, ids)
}

fn compile_multi_int(expressions: &[&str], ids: &[ReportId]) -> Result<(), CompileError> {
    for (i, (expression, &id)) in expressions.iter().zip(ids.iter()).enumerate() {
        add_expression(i as u32, expression, id)?;
    }
    Ok(())
}
