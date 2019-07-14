use crate::parser::*;
use crate::util::{make_caseless, CharReach};

/// Character classes and their mnemonics.
pub(in crate::parser) struct AsciiComponentClass {
    mode: ParseMode,
    cr: CharReach,
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

impl Component for AsciiComponentClass {}
