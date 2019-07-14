use std::ops::BitOrAssign;

use crate::util::{mytolower, BitField256};

pub(crate) struct CharReach {
    bits: BitField256,
}

impl CharReach {
    /// Constructs a character class containing a single char.
    pub(crate) fn from_char(c: u8) -> Self {
        let mut cr = CharReach::default();
        cr.set(c);
        cr
    }

    // Sets bit N.
    pub(crate) fn set(&mut self, c: u8) {
        self.bits.set(c);
    }

    // Tests bit N.
    pub(crate) fn test(&self, c: u8) -> bool {
        self.bits.test(c)
    }
}

impl Default for CharReach {
    fn default() -> Self {
        let bits = BitField256::default();
        CharReach { bits }
    }
}

impl BitOrAssign for CharReach {
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

pub(crate) fn make_caseless(cr: &mut CharReach) {
    for c in b'A'..=b'Z' {
        if cr.test(c) || cr.test(mytolower(c)) {
            cr.set(c);
            cr.set(mytolower(c));
        }
    }
}
