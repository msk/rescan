use super::ascii_component_class::AsciiComponentClass;
use super::{ComponentAlternation, ComponentSequence};

pub(crate) trait ConstComponentVisitor {
    type Error;

    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), Self::Error>;
    fn pre_component_alternation(&mut self, c: &ComponentAlternation) -> Result<(), Self::Error>;
    fn pre_component_sequence(&mut self, c: &ComponentSequence) -> Result<(), Self::Error>;

    fn during_ascii_component_class(&self, c: &AsciiComponentClass);
    fn during_component_alternation(&self, c: &ComponentAlternation);
    fn during_component_sequence(&self, c: &ComponentSequence);

    fn post_ascii_component_class(&mut self, c: &AsciiComponentClass);
    fn post_component_alternation(&mut self, c: &ComponentAlternation);
    fn post_component_sequence(&mut self, c: &ComponentSequence);
}
