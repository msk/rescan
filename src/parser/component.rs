use super::component_alternation::walk_component_alternation;
use super::component_class::walk_component_class;
use super::component_sequence::walk_component_sequence;
use super::GlushkovBuildState;
use super::{ComponentAlternation, ComponentClass, ComponentSequence, ConstComponentVisitor};

/// A component for a regular expression parse tree.
pub(crate) enum Component {
    Alternation(ComponentAlternation),
    Class(ComponentClass),
    Sequence(ComponentSequence),
}

impl Component {
    /// Informs the Glushkov build process of the positions used by this component.
    pub(crate) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        match self {
            Component::Alternation(c) => c.note_positions(bs),
            Component::Class(c) => c.note_positions(bs),
            Component::Sequence(c) => c.note_positions(bs),
        }
    }
}

impl From<ComponentClass> for Component {
    fn from(c: ComponentClass) -> Self {
        Component::Class(c)
    }
}

impl From<ComponentSequence> for Component {
    fn from(c: ComponentSequence) -> Self {
        Component::Sequence(c)
    }
}

/// Applies the given const visitor functor.
pub(crate) fn walk_component<V: ConstComponentVisitor>(
    v: &mut V,
    c: &Component,
) -> Result<(), V::Error> {
    match c {
        Component::Alternation(c) => walk_component_alternation(v, c),
        Component::Class(c) => walk_component_class(v, c),
        Component::Sequence(c) => walk_component_sequence(v, c),
    }
}
