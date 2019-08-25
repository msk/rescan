use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;

pub(crate) struct ComponentSequence {
    children: Vec<Box<dyn Component>>,
}

impl ComponentSequence {
    pub(in crate::parser) fn add_component(&mut self, comp: Box<dyn Component>) {
        self.children.push(comp);
    }
}

impl Component for ComponentSequence {
    fn accept(&self, _v: Box<&mut dyn ConstComponentVisitor>) -> Result<(), NotLiteral> {
        unimplemented!()
    }
}

impl Default for ComponentSequence {
    fn default() -> Self {
        ComponentSequence {
            children: Vec::new(),
        }
    }
}
