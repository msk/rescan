use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;

pub(crate) trait ConstComponentVisitor {
    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), NotLiteral>;
    fn pre_component_sequence(&self, c: &ComponentSequence) -> Result<(), NotLiteral>;
}
