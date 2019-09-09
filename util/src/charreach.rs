use crate::{mytolower, mytoupper, BitField256};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Character reachability.
///
/// This is a simple (but hopefully fast) struct for representing 8-bit
/// character reachability, along with a bunch of useful operations.
#[derive(Copy, Clone)]
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

    /// Constructs a character class representing a contiguous range of
    /// 8-bit characters, inclusively.
    ///
    /// # Panics
    ///
    /// Panics if `from > to`.
    pub fn from_range(from: u8, to: u8) -> Self {
        let mut cr = CharReach::default();
        cr.set_range(from, to);
        cr
    }

    /// Resets all bits.
    pub fn reset_all(&mut self) {
        self.bits.reset_all();
    }

    /// Sets all bits.
    pub fn set_all(&mut self) {
        self.bits.set_all();
    }

    /// Resets bit for `c`.
    pub fn reset(&mut self, c: u8) {
        self.bits.reset(c);
    }

    /// Resets bit for `c`.
    pub fn set(&mut self, c: u8) {
        self.bits.set(c);
    }

    /// Tests bit for `c`.
    pub fn test(&self, c: u8) -> bool {
        self.bits.test(c)
    }

    /// Flips all bits.
    pub fn flip_all(&mut self) {
        self.bits.flip_all();
    }

    /// Flips bit for `c`.
    pub fn flip(&mut self, c: u8) {
        self.bits.flip(c);
    }

    /// Switches on the bits in the range `from..=to`.
    ///
    /// # Panics
    ///
    /// Panics if `from > to`.
    pub fn set_range(&mut self, from: u8, to: u8) {
        self.bits.set_range(from, to)
    }

    /// Returns number of bits set on.
    pub fn count(&self) -> u32 {
        self.bits.count()
    }

    /// Returns `true` if no bit is set.
    pub fn none(&self) -> bool {
        self.bits.none()
    }

    /// Retruns `true` if any bit is set.
    pub fn any(&self) -> bool {
        self.bits.any()
    }

    /// Returns `true` if all bits are set.
    pub fn all(&self) -> bool {
        self.bits.all()
    }

    /// Returns first bit set.
    pub fn find_first(&self) -> Option<u8> {
        self.bits.find_first()
    }

    /// Returns last bit set.
    pub fn find_last(&self) -> Option<u8> {
        self.bits.find_last()
    }

    /// Returns next bit set
    pub fn find_next(&self, last: u8) -> Option<u8> {
        self.bits.find_next(last)
    }

    /// Returns (zero-based) `n`-th bit set.
    pub fn find_nth(&self, n: u8) -> Option<u8> {
        self.bits.find_nth(n)
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

impl fmt::Display for CharReach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let next = self.find_first();
        while let Some(pos) = next {
            s.push(pos as char);
        }
        write!(f, "{}", s)
    }
}

impl BitAnd for CharReach {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs &= self;
        rhs
    }
}

impl BitAndAssign for CharReach {
    #[inline]
    fn bitand_assign(&mut self, a: Self) {
        self.bits &= a.bits;
    }
}

impl BitOr for CharReach {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs |= self;
        rhs
    }
}

impl BitOrAssign for CharReach {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.bits |= rhs.bits;
    }
}

impl BitXor for CharReach {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs ^= self;
        rhs
    }
}

impl BitXorAssign for CharReach {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.bits ^= rhs.bits;
    }
}

impl Not for CharReach {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self::Output {
        self.bits = !self.bits;
        self
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

#[cfg(test)]
mod tests {
    use super::CharReach;

    #[test]
    fn init() {
        let cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        assert!(!cr.all());
    }

    #[test]
    fn set() {
        let mut cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        assert!(!cr.all());
        cr.set(b'q');
        assert_eq!(cr.count(), 1);
        cr.set_all();
        assert_eq!(cr.count(), 256);
        assert!(cr.all());
    }

    #[test]
    fn reset() {
        let mut cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        assert!(!cr.all());
        cr.set(b'q');
        cr.set(b'u');
        cr.set(b'a');
        cr.set(b'r');
        cr.set(b'k');
        assert_eq!(cr.count(), 5);
        cr.reset(b'r');
        assert_eq!(cr.count(), 4);
        assert!(!cr.test(b'r'));
        cr.set_all();
        assert_eq!(cr.count(), 256);
        assert!(cr.all());
        cr.reset(0xff);
        assert!(!cr.all());
    }

    #[test]
    fn flip() {
        let mut cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        cr.flip_all();
        assert_eq!(cr.count(), 256);
        assert!(cr.all());
        cr.flip_all();
        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        cr.flip(25);
        assert!(!cr.none());
        assert!(!cr.all());
        assert_eq!(cr.count(), 1);
        cr.flip_all();
        assert_eq!(cr.count(), 255);
    }

    #[test]
    fn count() {
        let mut cr = CharReach::default();

        cr.set(1);
        cr.set(2);
        cr.set(b'a');
        cr.set(b'Z');
        cr.set(b'm');
        cr.set(b'~');
        cr.set(210);

        let mut n = cr.find_first().expect("shoud not be empty");
        let mut i = 1;
        while let Some(next) = cr.find_next(n) {
            i += 1;
            n = next;
        }

        assert_eq!(cr.count(), i);
    }
}
