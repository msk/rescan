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
    fn accept(&self, v: &mut dyn ConstComponentVisitor) -> Result<(), NotLiteral> {
        v.pre_component_sequence(self);

        let mut child_iter = self.children.iter().peekable();
        while let Some(child) = child_iter.next() {
            child.accept(v)?;

            if child_iter.peek().is_some() {
                v.during_component_sequence(self);
            }
        }

        v.post_component_sequence(self);

        Ok(())
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
