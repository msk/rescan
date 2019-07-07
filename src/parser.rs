mod control_verbs;
mod parser_util;

use crate::parser::control_verbs::read_control_verbs;
use crate::util::compile_error::{CompileError, ErrorKind};
pub(crate) use parser_util::ParseMode;

pub(crate) fn parse(ptr: &str, global_mode: &mut ParseMode) -> Result<(), CompileError> {
    let p = ptr;

    let _p = read_control_verbs(p, 0, global_mode)?;
    Err(CompileError::new(ErrorKind::Other, "not implemented"))
}
