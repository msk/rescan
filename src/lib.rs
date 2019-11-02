mod compiler;
mod database;
mod grey;
mod nfagraph;
mod parser;
mod rose;
mod runtime;
mod scratch;
mod util;

use bitflags::bitflags;
use compiler::{add_expression, build};
pub use database::Database;
use grey::Grey;
use itertools::izip;
use nfagraph::Ng;
pub use rescan_util::ReportId;
pub use runtime::{scan, Error};
pub use scratch::Scratch;
pub use util::compile_error::{CompileError, ErrorKind};
use util::CompileContext;

/// Compile mode.
///
/// The mode is used as the mode parameter of the various compile calls
/// ([`compile`][`compile`] and [`compile_multi`][`compile_multi`]).
///
/// [`compile`]: fn.compile.html
/// [`compile_multi`]: fn.compile_multi.html
pub enum Mode {
    /// Block scan database.
    Block,
    /// Streaming database.
    Stream(Option<SomHorizon>),
    /// Vectored scanning database.
    Vectored,
}

impl Mode {
    fn is_streaming(&self) -> bool {
        match self {
            Self::Stream(_) | Self::Vectored => true,
            _ => false,
        }
    }

    fn is_vectored(&self) -> bool {
        match self {
            Self::Vectored => true,
            _ => false,
        }
    }

    /// Returns the number of bytes of SOM precision.
    fn som_precision(&self) -> usize {
        match self {
            Self::Stream(Some(horizon)) => match horizon {
                SomHorizon::Large => 8,
                SomHorizon::Medium => 4,
                SomHorizon::Small => 2,
            },
            Self::Vectored => 8,
            _ => 0,
        }
    }
}

pub enum SomHorizon {
    Large,
    Medium,
    Small,
}

/// Compiles a regular expression.
pub fn compile(expression: &str, flags: Flags, mode: Mode) -> Result<Database, CompileError> {
    compile_multi_int(&[expression], &[flags], &[0], mode, &Grey::default())
}

/// Compiles multiple regular expressions.
pub fn compile_multi(
    expressions: &[&str],
    flags: &[Flags],
    ids: &[ReportId],
    mode: Mode,
) -> Result<Database, CompileError> {
    compile_multi_int(expressions, flags, ids, mode, &Grey::default())
}

fn compile_multi_int(
    expressions: &[&str],
    flags: &[Flags],
    ids: &[ReportId],
    mode: Mode,
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

    let is_streaming = mode.is_streaming();
    let is_vectored = mode.is_vectored();
    let som_precision = mode.som_precision();

    let cc = CompileContext::new(is_streaming, is_vectored, g);
    let mut ng = Ng::new(cc, expressions.len(), som_precision);

    for (i, (exp, &fl, &id)) in izip!(expressions, flags, ids).enumerate() {
        let expression_index = i as u32;
        if let Err(mut e) = add_expression(&mut ng, expression_index, exp, fl, id) {
            e.set_expression_index(expression_index);
            return Err(e);
        }
    }

    Ok(build(&ng))
}

bitflags! {
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

#[cfg(test)]
mod tests {
    use super::{Grey, Mode};

    #[test]
    fn compile_single() {
        assert!(super::compile("foobar", super::Flags::empty(), Mode::Block).is_ok());
    }

    #[test]
    fn compile_multi_int_empty_input() {
        assert!(super::compile_multi_int(&[], &[], &[], Mode::Block, &Grey::default()).is_err());
    }
}
