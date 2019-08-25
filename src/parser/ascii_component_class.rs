use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;
use crate::util::{make_caseless, CharReach};

/// Character classes and their mnemonics.
pub(crate) struct AsciiComponentClass {
    mode: ParseMode,
    pub(in crate::parser) cr: CharReach,
}

impl AsciiComponentClass {
    pub(in crate::parser) fn new(mode: ParseMode) -> Self {
        AsciiComponentClass {
            mode,
            cr: CharReach::default(),
        }
    }
}

impl AsciiComponentClass {
    pub(in crate::parser) fn add(&mut self, c: u8) {
        let mut ncr = CharReach::from_char(c);
        if self.mode.caseless {
            make_caseless(&mut ncr);
        }

        self.cr |= ncr;
    }
}

impl Component for AsciiComponentClass {
    fn accept(&self, v: Box<&mut dyn ConstComponentVisitor>) -> Result<(), NotLiteral> {
        v.pre_ascii_component_class(self)?;
        Ok(())
    }
}
