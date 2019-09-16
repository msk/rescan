use super::ascii_component_class::AsciiComponentClass;
use crate::parser::*;
use crate::util::{describe_class, CcOutput};
use std::fmt;
use std::fmt::Write;

struct DumpVisitor<'w, W: Write> {
    out: &'w mut W,
    level: usize,
}

impl<'w, W: Write> DumpVisitor<'w, W> {
    fn new(out: &'w mut W) -> Self {
        DumpVisitor { out, level: 0 }
    }

    fn indent(&mut self) {
        self.level += 1;
    }

    fn outdent(&mut self) {
        debug_assert!(self.level > 0);
        self.level -= 1;
    }

    fn filler(&self) -> String {
        " ".repeat(self.level * 2)
    }
}

impl<'w, W: Write> ConstComponentVisitor for DumpVisitor<'w, W> {
    type Error = fmt::Error;

    fn pre_ascii_component_class(&mut self, c: &AsciiComponentClass) -> Result<(), Self::Error> {
        self.out
            .write_str(&(self.filler() + "ASCII CLASS\n" + &self.filler() + "  "))?;
        describe_class(self.out, &c.cr, 256, CcOutput::Text)?;
        self.out.write_char('\n')?;
        self.indent();
        Ok(())
    }

    fn post_ascii_component_class(&mut self, _c: &AsciiComponentClass) {
        self.outdent();
    }

    fn pre_component_alternation(&mut self, _c: &ComponentAlternation) -> Result<(), Self::Error> {
        self.out.write_str(&(self.filler() + "ALTERNATION\n"))?;
        self.indent();
        Ok(())
    }

    fn post_component_alternation(&mut self, _c: &ComponentAlternation) {
        self.outdent();
    }

    fn pre_component_sequence(&mut self, c: &ComponentSequence) -> Result<(), Self::Error> {
        self.out
            .write_str(&format!("{}{}", self.filler(), "SEQUENCE"))?;
        if let Some(index) = c.capture_index {
            self.out.write_str(&format!(" (capture index {})", index))?;
        } else {
            self.out.write_str(" (not captured)")?;
        }
        if let Some(name) = &c.capture_name {
            self.out
                .write_str(&format!(" (capture name '{}')\n", name))?;
        } else {
            self.out.write_char('\n')?;
        }
        self.indent();
        if c.children().is_empty() {
            self.out
                .write_str(&format!("{} <empty>\n", self.filler()))?;
        }
        Ok(())
    }

    fn post_component_sequence(&mut self, _c: &ComponentSequence) {
        self.outdent();
    }

    // not used
    fn during_ascii_component_class(&self, _c: &AsciiComponentClass) {}
    fn during_component_alternation(&self, _c: &ComponentAlternation) {}
    fn during_component_sequence(&self, _c: &ComponentSequence) {}
}

pub(crate) fn dump_tree<W: Write>(out: &mut W, root: &Component) -> Result<(), fmt::Error> {
    let mut vis = DumpVisitor::new(out);
    walk_component(&mut vis, root)
}
