use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;

pub(crate) trait ConstComponentVisitor {
    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), NotLiteral>;
    fn pre_component_sequence(&self, c: &ComponentSequence);

    fn during_ascii_component_class(&self, c: &AsciiComponentClass);
    fn during_component_sequence(&self, c: &ComponentSequence);

    fn post_ascii_component_class(&self, c: &AsciiComponentClass);
    fn post_component_sequence(&self, c: &ComponentSequence);
}
