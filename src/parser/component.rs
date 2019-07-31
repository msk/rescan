use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::{ConstComponentVisitor, GlushkovBuildState};

pub(crate) trait Component {
    /// Applies the given const visitor functor.
    fn accept(&self, v: Box<&mut dyn ConstComponentVisitor>) -> Result<(), NotLiteral>;

    /// Informs the Glushkov build process of the positions used by this component.
    fn note_positions(&mut self, bs: &mut GlushkovBuildState);
}
