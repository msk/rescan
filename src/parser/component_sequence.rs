use super::{
    walk_component, Component, ComponentAlternation, ConstComponentVisitor, GlushkovBuildState,
};
use std::mem;

#[derive(Default)]
pub(crate) struct ComponentSequence {
    children: Vec<Component>,
    alternation: Option<ComponentAlternation>,

    pub(super) capture_index: Option<u32>,
    #[allow(dead_code)]
    pub(super) capture_name: Option<String>,
}

impl ComponentSequence {
    pub(super) fn add_alternation(&mut self) {
        if self.alternation.is_none() {
            self.alternation = Some(ComponentAlternation::default());
        }
        let alternation = self.alternation.as_mut().expect("Some");

        let mut seq = ComponentSequence::default();
        mem::swap(&mut self.children, &mut seq.children);
        alternation.append(seq);
    }

    pub(in crate::parser) fn finalize(&mut self) {
        if self.alternation.is_some() {
            self.add_alternation();
            debug_assert!(self.children.is_empty());
            let alternation = self.alternation.take().expect("Some");
            self.children.push(Component::Alternation(alternation));
        }
    }

    pub(in crate::parser) fn add_component(&mut self, comp: Component) {
        self.children.push(comp);
    }

    /// Informs the Glushkov build process of the positions used by this component.
    pub(in crate::parser) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        for c in self.children.iter_mut() {
            c.note_positions(bs);
        }
    }

    #[cfg(test)]
    pub(super) fn children(&self) -> &Vec<Component> {
        &self.children
    }
}

pub(in crate::parser) fn walk_component_sequence<V: ConstComponentVisitor>(
    v: &mut V,
    c: &ComponentSequence,
) -> Result<(), V::Error> {
    debug_assert!(c.alternation.is_none()); // Sequence must be finalized first.

    v.pre_component_sequence(c)?;

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
