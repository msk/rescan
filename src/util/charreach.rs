use std::ops::BitOrAssign;

use crate::util::{mytolower, mytoupper, BitField256};

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

    /// Sets all bits.
    pub(crate) fn setall(&mut self) {
        self.bits.setall();
    }

    /// Sets bit N.
    pub(crate) fn set(&mut self, c: u8) {
        self.bits.set(c);
    }

    /// Tests bit N.
    pub(crate) fn test(&self, c: u8) -> bool {
        self.bits.test(c)
    }

    /// Returns number of bits set on.
    pub(crate) fn count(&self) -> usize {
        self.bits.count()
    }

    /// Returns first bit set.
    pub(crate) fn find_first(&self) -> Option<u8> {
        self.bits.find_first()
    }

    /// Returns next bit set
    pub(crate) fn find_next(&self, last: u8) -> Option<u8> {
        self.bits.find_next(last)
    }

    pub(crate) fn bitor(&mut self, rhs: &Self) {
        self.bits.bitor(&rhs.bits);
    }

    /// Returns `true` if this represents an uppercase/lowercase pair.
    pub(crate) fn is_caseless_char(&self) -> bool {
        if self.count() != 2 {
            return false;
        }
        let first = self.find_first().expect("should have two bits set");
        let second = self.find_next(first).expect("should have two bits set");
        first == mytoupper(second)
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
