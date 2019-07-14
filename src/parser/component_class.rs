use crate::parser::*;
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

pub(in crate::parser) enum ComponentClass {
    Ascii(AsciiComponentClass),
}

impl ComponentClass {
    fn add(&mut self, c: char) -> Result<(), CompileError> {
        match self {
            ComponentClass::Ascii(cc) => {
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
}

impl From<ComponentClass> for Box<dyn Component> {
    fn from(cc: ComponentClass) -> Self {
        match cc {
            ComponentClass::Ascii(cc) => Box::new(cc),
        }
    }
}
