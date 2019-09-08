use crate::{mytolower, mytoupper, BitField256};
use std::ops::BitOrAssign;

/// Character reachability.
///
/// This is a simple (but hopefully fast) struct for representing 8-bit
/// character reachability, along with a bunch of useful operations.
pub struct CharReach {
    /// Underlying storage.
    bits: BitField256,
}

impl CharReach {
    /// Constructs a character class containing a single 8-bit character.
    pub fn from_char(c: u8) -> Self {
        let mut cr = CharReach::default();
        cr.set(c);
        cr
    }

    /// Sets all bits.
    pub fn setall(&mut self) {
        self.bits.setall();
    }

    /// Clears all bits.
    pub fn clear(&mut self) {
        self.bits.clear();
    }

    /// Sets bit N.
    pub fn set(&mut self, c: u8) {
        self.bits.set(c);
    }

    /// Tests bit N.
    pub fn test(&self, c: u8) -> bool {
        self.bits.test(c)
    }

    /// Switches on the bits in the range `from..=to`.
    pub fn set_range(&mut self, from: u8, to: u8) {
        self.bits.set_range(from, to)
    }

    /// Returns number of bits set on.
    pub fn count(&self) -> u8 {
        self.bits.count()
    }

    /// Returns first bit set.
    pub fn find_first(&self) -> Option<u8> {
        self.bits.find_first()
    }

    /// Returns next bit set
    pub fn find_next(&self, last: u8) -> Option<u8> {
        self.bits.find_next(last)
    }

    pub fn bitor(&mut self, rhs: &Self) {
        self.bits.bitor(&rhs.bits);
    }

    /// Returns `true` if this represents an uppercase/lowercase pair.
    pub fn is_caseless_char(&self) -> bool {
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

pub fn make_caseless(cr: &mut CharReach) {
    for c in b'A'..=b'Z' {
        if cr.test(c) || cr.test(mytolower(c)) {
            cr.set(c);
            cr.set(mytolower(c));
        }
    }
}
