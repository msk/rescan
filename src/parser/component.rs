use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::ConstComponentVisitor;

pub(crate) trait Component {
    fn accept(&self, v: Box<&mut dyn ConstComponentVisitor>) -> Result<(), NotLiteral>;
}
