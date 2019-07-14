use crate::parser::Component;

pub(in crate::parser) struct ComponentSequence {
    children: Vec<Box<dyn Component>>,
}

impl ComponentSequence {
    pub(in crate::parser) fn add_component(&mut self, comp: Box<dyn Component>) {
        self.children.push(comp);
    }
}

impl Default for ComponentSequence {
    fn default() -> Self {
        ComponentSequence {
            children: Vec::new(),
        }
    }
}
