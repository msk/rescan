mod compiler;
mod database;
mod grey;
mod nfagraph;
mod parser;
mod rose;
mod runtime;
mod ue2common;
mod util;

use compiler::{add_expression, build};
pub use database::Database;
use grey::Grey;
use nfagraph::Ng;
pub use runtime::scan;
pub use ue2common::ReportId;
pub use util::compile_error::{CompileError, ErrorKind};
use util::CompileContext;

/// Compiles a regular expression.
pub fn compile(expression: &str) -> Result<Database, CompileError> {
    compile_multi_int(&[expression], &[0], &Grey::default())
}

/// Compiles multiple regular expressions.
pub fn compile_multi(expressions: &[&str], ids: &[ReportId]) -> Result<Database, CompileError> {
    compile_multi_int(expressions, ids, &Grey::default())
}

fn compile_multi_int(
    expressions: &[&str],
    ids: &[ReportId],
    g: &Grey,
) -> Result<Database, CompileError> {
    if expressions.is_empty() {
        return Err(CompileError::new(
            ErrorKind::Other,
            "Invalid parameter: expressions is empty",
        ));
    }

    if expressions.len() > g.limit_pattern_count {
        return Err(CompileError::new(
            ErrorKind::Other,
            "Number of patterns too large",
        ));
    }

    let cc = CompileContext::new(g);
    let mut ng = Ng::new(cc);

    for (i, (expression, &id)) in expressions.iter().zip(ids.iter()).enumerate() {
        add_expression(&mut ng, i as u32, expression, id)?;
    }

    Ok(build(&ng))
}

#[cfg(test)]
mod tests {
    use super::Grey;

    #[test]
    fn compile_single() {
        assert!(super::compile("foobar").is_ok());
    }

    #[test]
    fn compile_multi_int_empty_input() {
        assert!(super::compile_multi_int(&[], &[], &Grey::default()).is_err());
    }
}
