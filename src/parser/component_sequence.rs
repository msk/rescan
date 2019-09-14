use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;

pub(crate) struct ComponentSequence {
    children: Vec<Box<dyn Component>>,
    alternation: Option<()>,

    capture_index: Option<u32>,
}

impl ComponentSequence {
    pub(in crate::parser) fn finalize(&mut self) {
        if self.alternation.is_some() {
            unimplemented!();
        }
    }

    pub(in crate::parser) fn add_component(&mut self, comp: Box<dyn Component>) {
        self.children.push(comp);
    }

    pub(crate) fn set_capture_index(&mut self, idx: u32) {
        self.capture_index = Some(idx);
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
            alternation: None,
            capture_index: None,
        }
    }
}
