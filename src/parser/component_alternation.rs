use super::component_sequence::walk_component_sequence;
use super::{ComponentSequence, ConstComponentVisitor, GlushkovBuildState};

#[derive(Default)]
pub(crate) struct ComponentAlternation {
    children: Vec<ComponentSequence>,
}

impl ComponentAlternation {
    pub(super) fn append(&mut self, component: ComponentSequence) {
        self.children.push(component);
    }

    /// Informs the Glushkov build process of the positions used by this component.
    pub(super) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        for c in &mut self.children {
            c.note_positions(bs);
        }
    }
}

pub(super) fn walk_component_alternation<V: ConstComponentVisitor>(
    v: &mut V,
    c: &ComponentAlternation,
) -> Result<(), V::Error> {
    v.pre_component_alternation(c)?;

    let mut child_iter = c.children.iter().peekable();
    while let Some(child) = child_iter.next() {
        walk_component_sequence(v, child)?;

        if child_iter.peek().is_some() {
            v.during_component_alternation(c);
        }
    }

    v.post_component_alternation(c);

    Ok(())
}
