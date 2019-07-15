use crate::compiler::ParsedExpression;
use crate::nfagraph::Ng;
use crate::parser::*;
use crate::util::Ue2Literal;

pub(crate) struct NotLiteral {}

#[derive(Default)]
struct ConstructLiteralVisitor {
    lit: Ue2Literal,
}

impl ConstComponentVisitor for ConstructLiteralVisitor {
    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), NotLiteral> {
        let cr = &c.cr;
        let width = cr.count();
        if width == 1 {
            self.lit
                .push(cr.find_first().expect("should have one bit set"), false);
        } else if width == 2 && cr.is_caseless_char() {
            self.lit
                .push(cr.find_first().expect("should have two bits set"), true);
        } else {
            return Err(NotLiteral {});
        }
        Ok(())
    }

    fn pre_component_sequence(&self, _c: &ComponentSequence) -> Result<(), NotLiteral> {
        // Pass through.
        Ok(())
    }
}

pub(crate) fn shortcut_literal(ng: &mut Ng, pe: &ParsedExpression) -> bool {
    let mut vis = ConstructLiteralVisitor::default();
    if let Err(_not_literal) = pe.component.accept(Box::new(&mut vis)) {
        return false;
    }

    vis.lit.set_pure();
    let lit = &vis.lit;

    if lit.is_empty() {
        return false;
    }

    ng.add_literal(lit)
}
