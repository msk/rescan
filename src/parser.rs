mod ascii_component_class;
mod build_state;
mod component;
mod component_class;
mod component_sequence;
mod const_component_visitor;
mod control_verbs;
mod parser_util;
mod position;
mod position_info;
mod prefilter;
mod shortcut_literal;

pub(in crate::parser) use ascii_component_class::AsciiComponentClass;
pub(crate) use build_state::make_glushkov_build_state;
pub(in crate::parser) use build_state::GlushkovBuildState;
pub(crate) use component::Component;
pub(in crate::parser) use component_class::get_literal_component_class;
pub(in crate::parser) use component_sequence::ComponentSequence;
pub(in crate::parser) use const_component_visitor::ConstComponentVisitor;
pub(crate) use parser_util::ParseMode;
pub(crate) use position::{PosFlags, Position};
pub(crate) use position_info::PositionInfo;
pub(crate) use prefilter::prefilter_tree;
pub(crate) use shortcut_literal::shortcut_literal;

use nom::{bytes::complete::take, IResult};

use crate::parser::control_verbs::read_control_verbs;
use crate::util::compile_error::CompileError;

fn add_literal(
    current_seq: &mut ComponentSequence,
    c: char,
    mode: ParseMode,
) -> Result<(), CompileError> {
    let cc = get_literal_component_class(c, mode.caseless)?;
    current_seq.add_component(cc.into());
    Ok(())
}

pub(crate) fn parse(
    ptr: &str,
    global_mode: &mut ParseMode,
) -> Result<Box<dyn Component>, CompileError> {
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
    Ok(Box::new(root_seq))
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
