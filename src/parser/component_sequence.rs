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

    fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        let _pb = bs.get_builder().num_vertices();
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
