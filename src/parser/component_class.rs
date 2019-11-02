use super::ascii_component_class::{walk_ascii_component_class, AsciiComponentClass};
use super::{ConstComponentVisitor, GlushkovBuildState, ParseMode};
use crate::util::compile_error::{CompileError, ErrorKind};

pub(in crate::parser) fn get_component_class(mode: ParseMode) -> ComponentClass {
    ComponentClass::Ascii(AsciiComponentClass::new(mode))
}

/// Generates a component for a single literal character, possibly in caseless
/// mode.
pub(in crate::parser) fn get_literal_component_class(
    c: char,
    nocase: bool,
) -> Result<ComponentClass, CompileError> {
    let mut mode = ParseMode::default();
    mode.caseless = nocase;
    let mut cc = get_component_class(mode);
    cc.add(c)?;
    Ok(cc)
}

#[derive(Debug)]
pub(crate) enum ComponentClass {
    Ascii(AsciiComponentClass),
}

impl ComponentClass {
    fn add(&mut self, c: char) -> Result<(), CompileError> {
        match self {
            Self::Ascii(cc) => {
                if c > '\u{ff}' {
                    return Err(CompileError::new(
                        ErrorKind::LocatedParse,
                        "Hexadecimal value is greater than \\xFF".to_string(),
                    ));
                }
                cc.add(c as u8)
            }
        }
        Ok(())
    }

    /// Informs the Glushkov build process of the positions used by this component.
    pub(in crate::parser) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        match self {
            Self::Ascii(c) => c.note_positions(bs),
        }
    }
}

/// Applies the given const visitor functor.
pub(crate) fn walk_component_class<V: ConstComponentVisitor>(
    v: &mut V,
    c: &ComponentClass,
) -> Result<(), V::Error> {
    match c {
        ComponentClass::Ascii(c) => walk_ascii_component_class(v, c),
    }
}
