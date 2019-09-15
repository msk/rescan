use super::ascii_component_class::AsciiComponentClass;
use crate::compiler::ParsedExpression;
use crate::nfagraph::Ng;
use crate::parser::{walk_component, ComponentSequence, ConstComponentVisitor};
use rescan_util::Ue2Literal;

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

    fn pre_component_sequence(&self, _c: &ComponentSequence) {
        // Pass through.
    }

    fn during_ascii_component_class(&self, _c: &AsciiComponentClass) {}
    fn during_component_sequence(&self, _c: &ComponentSequence) {}

    fn post_ascii_component_class(&self, _c: &AsciiComponentClass) {}
    fn post_component_sequence(&self, _c: &ComponentSequence) {}
}

pub(crate) fn shortcut_literal(ng: &mut Ng, pe: &ParsedExpression) -> bool {
    let mut vis = ConstructLiteralVisitor::default();
    if let Err(_not_literal) = walk_component(&mut vis, &pe.component) {
        return false;
    }

    vis.lit.set_pure();
    let lit = &vis.lit;

    if lit.is_empty() {
        return false;
    }

    ng.add_literal(lit)
}
