mod compiler;
mod database;
mod grey;
mod nfagraph;
mod parser;
mod rose;
mod runtime;
mod scratch;
mod ue2common;
mod util;

use bitflags::bitflags;
use compiler::{add_expression, build};
pub use database::Database;
use grey::Grey;
use itertools::izip;
use nfagraph::Ng;
pub use runtime::scan;
pub use scratch::Scratch;
pub use ue2common::ReportId;
pub use util::compile_error::{CompileError, ErrorKind};
use util::CompileContext;

/// Compiles a regular expression.
pub fn compile(expression: &str, flags: Flags) -> Result<Database, CompileError> {
    compile_multi_int(&[expression], &[flags], &[0], &Grey::default())
}

/// Compiles multiple regular expressions.
pub fn compile_multi(
    expressions: &[&str],
    flags: &[Flags],
    ids: &[ReportId],
) -> Result<Database, CompileError> {
    compile_multi_int(expressions, flags, ids, &Grey::default())
}

fn compile_multi_int(
    expressions: &[&str],
    flags: &[Flags],
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

    for (i, (exp, &fl, &id)) in izip!(expressions, flags, ids).enumerate() {
        add_expression(&mut ng, i as u32, exp, fl, id)?;
    }

    Ok(build(&ng))
}

bitflags! {
    #[derive(Default)]
    pub struct Flags : u16 {
        const CASELESS = 0x0001;
        const DOTALL = 0x0002;
        const MULTILINE = 0x0004;
        const SINGLEMATCH = 0x0008;
        const ALLOWEMPTY = 0x0010;
        const UTF8 = 0x0020;
        const UCP = 0x0040;
        const PREFILTER = 0x0080;
        const SOM_LEFTMOST = 0x0100;
        const COMBINATION = 0x0200;
        const QUIET = 0x0400;
    }
}

pub enum Error {}

#[cfg(test)]
mod tests {
    use super::Grey;

    #[test]
    fn compile_single() {
        assert!(super::compile("foobar", super::Flags::default()).is_ok());
    }

    #[test]
    fn compile_multi_int_empty_input() {
        assert!(super::compile_multi_int(&[], &[], &[], &Grey::default()).is_err());
    }
}
