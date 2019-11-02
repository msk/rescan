mod ascii_component_class;
mod build_state;
mod component;
mod component_alternation;
mod component_class;
mod component_sequence;
mod const_component_visitor;
mod control_verbs;
#[cfg(test)]
mod dump;
mod parser_util;
mod position;
mod position_info;
mod prefilter;
mod shortcut_literal;

pub(crate) use build_state::make_glushkov_build_state;
pub(crate) use component::Component;
pub(crate) use parser_util::ParseMode;
pub(crate) use position::{PosFlags, Position};
pub(crate) use position_info::PositionInfo;
pub(crate) use prefilter::prefilter_tree;
pub(crate) use shortcut_literal::shortcut_literal;

pub(in crate::parser) use build_state::GlushkovBuildState;
pub(in crate::parser) use component::walk_component;
pub(in crate::parser) use component_alternation::ComponentAlternation;
pub(in crate::parser) use component_class::{get_literal_component_class, ComponentClass};
pub(in crate::parser) use component_sequence::ComponentSequence;
pub(in crate::parser) use const_component_visitor::ConstComponentVisitor;

use crate::parser::control_verbs::read_control_verbs;
use crate::util::compile_error::{CompileError, ErrorKind};
use nom::{bytes::complete::take, character::complete::char, IResult};
use std::mem;

/// Structure representing current state as we're parsing (current sequence,
/// current options). Stored in the 'sequences' vector.
struct ExprState {
    seq: ComponentSequence, // Current sequence
    offset: usize,          // Offset seq was entered, for error reporting
    mode: ParseMode,        // Current mode flags
}

impl ExprState {
    fn new(seq: ComponentSequence, offset: usize, mode: ParseMode) -> Self {
        Self { seq, offset, mode }
    }
}

/// Adds a literal to the current sequence.
fn add_literal(
    current_seq: &mut ComponentSequence,
    c: char,
    mode: ParseMode,
) -> Result<(), CompileError> {
    let cc = get_literal_component_class(c, mode.caseless)?;
    current_seq.add_component(cc.into());
    Ok(())
}

struct Context<'p> {
    ptr: &'p str,
    p: &'p str,
    mode: ParseMode,

    /// Stack of sequences and flags used to store state when we enter
    /// sub-sequences.
    sequences: Vec<ExprState>,

    /// Index of the next capturing group.
    ///
    /// Note that zero is reserved for the root sequence.
    group_index: u32,

    /// Current sequence being appended to.
    current_seq: ComponentSequence,
}

impl<'p> Context<'p> {
    fn new(ptr: &'p str, p: &'p str, mode: ParseMode) -> Self {
        let mut current_seq = ComponentSequence::default();
        current_seq.capture_index = Some(0);

        Self {
            ptr,
            p,
            mode,
            sequences: Vec::new(),
            group_index: 1,
            current_seq,
        }
    }

    fn push_sequence(&mut self, ts: &'p str) {
        let mut seq = ComponentSequence::default();
        seq.capture_index = Some(self.group_index);
        self.group_index += 1;
        mem::swap(&mut self.current_seq, &mut seq);
        self.sequences
            .push(ExprState::new(seq, ts.len() - self.ptr.len(), self.mode));
    }

    fn pop_sequence(&mut self) -> Result<(), CompileError> {
        let (mut seq, mode) = if let Some(v) = self.sequences.pop() {
            (v.seq, v.mode)
        } else {
            return Err(CompileError::new(
                ErrorKind::LocatedParse,
                "Unmatched parentheses",
            ));
        };
        mem::swap(&mut self.current_seq, &mut seq);
        seq.finalize();
        self.current_seq.add_component(seq.into());
        self.mode = mode;
        Ok(())
    }

    fn enter_capturing_group(&mut self, ts: &'p str) {
        self.push_sequence(ts);
    }

    fn exit_group(&mut self) -> Result<(), CompileError> {
        self.pop_sequence()?;
        Ok(())
    }

    fn main(&mut self, ts: &'p str) -> Result<(), CompileError> {
        if let Ok((p, _)) = char::<&str, ()>('(')(ts) {
            self.enter_capturing_group(ts);
            self.p = p;
        } else if let Ok((p, _)) = char::<&str, ()>(')')(ts) {
            self.exit_group()?;
            self.p = p;
        } else if let Ok((p, _)) = char::<&str, ()>('|')(ts) {
            self.current_seq.add_alternation();
            self.p = p;
        } else if let Ok((p, c)) = take_any(ts) {
            // TODO: Support UTF-8 literals
            assert!(c.is_ascii());
            if let Err(mut e) = add_literal(&mut self.current_seq, c, self.mode) {
                e.reason
                    .push_str(&format!(" at index {}", self.ptr.len() - self.p.len()));
                return Err(e);
            }
            self.p = p
        }
        Ok(())
    }

    fn parse(mut self) -> Result<ComponentSequence, CompileError> {
        while !self.p.is_empty() {
            self.main(self.p)?;
        }

        if let Some(seq) = self.sequences.last() {
            return Err(CompileError::new(
                ErrorKind::Parse,
                format!(
                    "Missing close parenthesis for group started at index {}.",
                    seq.offset
                ),
            ));
        }

        // Finalize the top-level sequence, which will take care of any
        // top-level alternation.
        self.current_seq.finalize();
        Ok(self.current_seq)
    }
}

pub(crate) fn parse(ptr: &str, global_mode: &mut ParseMode) -> Result<Component, CompileError> {
    let p = read_control_verbs(ptr, 0, global_mode)?;

    let root_seq = Context::new(ptr, p, *global_mode).parse()?;

    Ok(Component::Sequence(root_seq))
}

fn take_any(input: &str) -> IResult<&str, char> {
    take(1_usize)(input).map(|(input, c)| {
        let c = c
            .chars()
            .next()
            .expect("take(1usize) in take_any should return a non-empty string.");
        (input, c)
    })
}

#[cfg(test)]
mod tests {
    use super::dump::dump_tree;
    use super::*;

    #[test]
    fn parse_alternation() {
        let mut mode = ParseMode::default();
        let c = parse("|", &mut mode).expect("valid");
        let mut out = String::new();
        dump_tree(&mut out, &c).unwrap();
        assert_eq!(
            out,
            "SEQUENCE (capture index 0)
  ALTERNATION
    SEQUENCE (not captured)
       <empty>
    SEQUENCE (not captured)
       <empty>
"
        );
    }

    #[test]
    fn parse_any() {
        let mut mode = ParseMode::default();
        let c = parse("a", &mut mode).expect("valid");
        let mut out = String::new();
        dump_tree(&mut out, &c).unwrap();
        assert_eq!(
            out,
            "SEQUENCE (capture index 0)
  ASCII CLASS
    a
"
        );
    }

    #[test]
    fn parse_capturing_group() {
        let mut mode = ParseMode::default();
        let c = parse("()", &mut mode).expect("valid");
        let mut out = String::new();
        dump_tree(&mut out, &c).unwrap();
        assert_eq!(
            out,
            "SEQUENCE (capture index 0)
  SEQUENCE (capture index 1)
     <empty>
"
        );
    }
}
