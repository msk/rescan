use crate::parser::shortcut_literal::NotLiteral;
use crate::parser::*;
use rescan_util::{make_caseless, CharReach};

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

    pub(in crate::parser) fn add(&mut self, c: u8) {
        let mut ncr = CharReach::from_char(c);
        if self.mode.caseless {
            make_caseless(&mut ncr);
        }

        self.cr |= ncr;
    }

    pub(in crate::parser) fn note_positions(&mut self, bs: &mut GlushkovBuildState) {
        let builder = bs.get_builder_mut();
        let position = builder.make_position();

        builder.add_char_reach(position, self.cr);
    }
}

pub(in crate::parser) fn walk_ascii_component_class<V: ConstComponentVisitor>(
    v: &mut V,
    c: &AsciiComponentClass,
) -> Result<(), NotLiteral> {
    v.pre_ascii_component_class(c)?;
    Ok(())
}
