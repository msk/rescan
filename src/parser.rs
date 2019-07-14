mod ascii_component_class;
mod component;
mod component_class;
mod component_sequence;
mod control_verbs;
mod parser_util;

use nom::{bytes::complete::take, IResult};

use crate::parser::control_verbs::read_control_verbs;
use crate::util::compile_error::CompileError;
pub(in crate::parser) use ascii_component_class::AsciiComponentClass;
pub(in crate::parser) use component::Component;
pub(in crate::parser) use component_class::get_literal_component_class;
pub(in crate::parser) use component_sequence::ComponentSequence;
pub(crate) use parser_util::ParseMode;

fn add_literal(
    current_seq: &mut ComponentSequence,
    c: char,
    mode: ParseMode,
) -> Result<(), CompileError> {
    let cc = get_literal_component_class(c, mode.caseless)?;
    current_seq.add_component(cc.into());
    Ok(())
}

pub(crate) fn parse(ptr: &str, global_mode: &mut ParseMode) -> Result<(), CompileError> {
    let p = ptr;

    let mut p = read_control_verbs(p, 0, global_mode)?;

    let mode = *global_mode;

    let mut root_seq = ComponentSequence::default();

    let current_seq = &mut root_seq;

    while !p.is_empty() {
        if let Ok((input, c)) = take_any(p) {
            // TODO: Support UTF-8 literals
            assert!(c.is_ascii());
            if let Err(mut e) = add_literal(current_seq, c, mode) {
                e.reason = e.reason + &format!(" at index {}", ptr.len() - p.len());
                return Err(e);
            }
            p = input
        }
    }
    Ok(())
}

fn take_any(input: &str) -> IResult<&str, char> {
    take(1usize)(input).map(|(input, c)| {
        let c = c
            .chars()
            .next()
            .expect("take(1usize) in take_any should return a non-empty string.");
        (input, c)
    })
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
