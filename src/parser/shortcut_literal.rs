use super::ascii_component_class::AsciiComponentClass;
use crate::compiler::ParsedExpression;
use crate::nfagraph::Ng;
use crate::parser::walk_component;
use crate::parser::{ComponentAlternation, ComponentSequence, ConstComponentVisitor};
use crate::util::Ue2Literal;
use crate::CompileError;
use std::convert::TryInto;
use std::pin::Pin;

pub(crate) struct NotLiteral {}

#[derive(Default)]
struct ConstructLiteralVisitor {
    lit: Ue2Literal,
}

impl ConstComponentVisitor for ConstructLiteralVisitor {
    type Error = NotLiteral;

    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), Self::Error> {
        let cr = &c.cr;
        let width = cr.count();
        if width == 1 {
            self.lit
                .push(cr.find_first().expect("should have one bit set"), false);
        } else if width == 2 && cr.is_caseless_char() {
            self.lit
                .push(cr.find_first().expect("should have two bits set"), true);
        } else {
            return Err(Self::Error {});
        }
        Ok(())
    }

    fn pre_component_sequence(&mut self, _c: &ComponentSequence) -> Result<(), Self::Error> {
        // Pass through.
        Ok(())
    }

    fn pre_component_alternation(&mut self, _c: &ComponentAlternation) -> Result<(), Self::Error> {
        Err(NotLiteral {})
    }

    fn during_ascii_component_class(&self, _c: &AsciiComponentClass) {}
    fn during_component_alternation(&self, _c: &ComponentAlternation) {}
    fn during_component_sequence(&self, _c: &ComponentSequence) {}

    fn post_ascii_component_class(&mut self, _c: &AsciiComponentClass) {}
    fn post_component_alternation(&mut self, _c: &ComponentAlternation) {}
    fn post_component_sequence(&mut self, _c: &ComponentSequence) {}
}

/// Returns `true` if the literal expression could be added to Rose.
pub(crate) fn shortcut_literal(
    ng: &mut Pin<Box<Ng>>,
    pe: &ParsedExpression,
) -> Result<bool, CompileError> {
    if !ng.cc.grey.allow_literal {
        return Ok(false);
    }

    let expr = &pe.expr;

    let mut vis = ConstructLiteralVisitor::default();
    if let Err(_not_literal) = walk_component(&mut vis, &pe.component) {
        return Ok(false);
    }

    let lit = &vis.lit;

    if lit.is_empty() {
        return Ok(false);
    }

    unsafe {
        ng.as_mut().get_unchecked_mut().add_literal(
            lit,
            expr.index.try_into().unwrap(),
            expr.report,
            expr.highlander,
            expr.som,
            expr.quiet,
        )
    }
}
