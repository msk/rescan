use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;

pub(crate) struct ComponentSequence {
    children: Vec<Component>,
    alternation: Option<()>,

    capture_index: Option<u32>,
}

impl ComponentSequence {
    pub(in crate::parser) fn finalize(&mut self) {
        if self.alternation.is_some() {
            unimplemented!();
        }
    }

    pub(in crate::parser) fn add_component(&mut self, comp: Component) {
        self.children.push(comp);
    }

    /// Informs the Glushkov build process of the positions used by this component.
    pub(in crate::parser) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        let _pb = bs.get_builder().num_vertices();
        for c in self.children.iter_mut() {
            c.note_positions(bs);
        }
    }

    pub(crate) fn set_capture_index(&mut self, idx: u32) {
        self.capture_index = Some(idx);
    }
}

pub(in crate::parser) fn walk_component_sequence<V: ConstComponentVisitor>(
    v: &mut V,
    c: &ComponentSequence,
) -> Result<(), NotLiteral> {
    v.pre_component_sequence(c);

    let mut child_iter = c.children.iter().peekable();
    while let Some(child) = child_iter.next() {
        walk_component(v, child)?;

        if child_iter.peek().is_some() {
            v.during_component_sequence(c);
        }
    }

    v.post_component_sequence(c);

    Ok(())
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
