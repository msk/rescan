mod control_verbs;
mod parser_util;

use nom::{bytes::complete::take, IResult};

use crate::parser::control_verbs::read_control_verbs;
use crate::util::compile_error::CompileError;
pub(crate) use parser_util::ParseMode;

pub(crate) fn parse(ptr: &str, global_mode: &mut ParseMode) -> Result<(), CompileError> {
    let p = ptr;

    let mut p = read_control_verbs(p, 0, global_mode)?;

    while !p.is_empty() {
        if let Ok((input, _c)) = take_any(p) {
            p = input
        }
    }
    Ok(())
}

fn take_any(input: &str) -> IResult<&str, &str> {
    take(1usize)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_any() {
        let mut mode = ParseMode::default();
        assert!(parse("abc", &mut mode).is_ok());
    }
}
