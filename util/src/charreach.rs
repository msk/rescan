use crate::{mytolower, mytoupper, ourisalpha, BitField256, UTF_CONT_MAX};
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// Character reachability.
///
/// This is a simple (but hopefully fast) struct for representing 8-bit
/// character reachability, along with a bunch of useful operations.
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct CharReach {
    /// Underlying storage.
    bits: BitField256,
}

impl CharReach {
    /// Constructs a character class containing a single 8-bit character.
    #[must_use]
    pub fn from_char(c: u8) -> Self {
        let mut cr = CharReach::default();
        cr.set(c);
        cr
    }

    /// Constructs a character class representing a contiguous range of 8-bit
    /// characters, inclusively.
    ///
    /// # Panics
    ///
    /// Panics if `from > to`.
    #[must_use]
    pub fn from_range(from: u8, to: u8) -> Self {
        let mut cr = CharReach::default();
        cr.set_range(from, to);
        cr
    }

    /// Constructs a character class based on the set of chars in a byte slice.
    #[must_use]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut cr = CharReach::default();
        cr.set_bytes(bytes);
        cr
    }

    /// Constructs a character class with complete reachability (a "dot").
    #[must_use]
    pub fn dot() -> Self {
        Self::from_range(0, 255)
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

    /// Sets bit for `c`.
    pub fn set(&mut self, c: u8) {
        self.bits.set(c);
    }

    /// Tests bit for `c`.
    #[must_use]
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

    /// Switches on the bits corresponding to the characters in `bytes`.
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.set(b);
        }
    }

    /// Returns number of bits set on.
    #[must_use]
    pub fn count(&self) -> u32 {
        self.bits.count()
    }

    /// Returns `true` if no bit is set.
    #[must_use]
    pub fn none(&self) -> bool {
        self.bits.none()
    }

    /// Retruns `true` if any bit is set.
    #[must_use]
    pub fn any(&self) -> bool {
        self.bits.any()
    }

    /// Returns `true` if all bits are set.
    #[must_use]
    pub fn all(&self) -> bool {
        self.bits.all()
    }

    /// Returns first bit set.
    #[must_use]
    pub fn find_first(&self) -> Option<u8> {
        self.bits.find_first()
    }

    /// Returns last bit set.
    #[must_use]
    pub fn find_last(&self) -> Option<u8> {
        self.bits.find_last()
    }

    /// Returns next bit set
    #[must_use]
    pub fn find_next(&self, last: u8) -> Option<u8> {
        self.bits.find_next(last)
    }

    /// Returns (zero-based) `n`-th bit set.
    #[must_use]
    pub fn find_nth(&self, n: u8) -> Option<u8> {
        self.bits.find_nth(n)
    }

    /// Checks if this only contain bits representing alphabet characters.
    #[must_use]
    pub fn is_alphabetic(&self) -> bool {
        if self.none() {
            return false;
        }
        let mut next = self.find_first();
        while let Some(pos) = next {
            if !ourisalpha(pos) {
                return false;
            }
            next = self.find_next(pos);
        }
        true
    }

    /// Checks if this represents an uppercase/lowercase pair.
    #[must_use]
    pub fn is_caseless_char(&self) -> bool {
        if self.count() != 2 {
            return false;
        }
        let first = self.find_first().expect("should have two bits set");
        let second = self.find_next(first).expect("should have two bits set");
        first == mytoupper(second)
    }

    /// Checks if this represents a cheapskate caseless set.
    #[must_use]
    pub fn is_bit5_insensitive(&self) -> bool {
        let mut next = self.find_first();
        while let Some(pos) = next {
            if !self.test(pos ^ 0x20) {
                return false;
            }
            next = self.find_next(pos);
        }
        true
    }

    /// Checks if this character class is a subset of `rhs`.
    #[must_use]
    pub fn is_subset_of(&self, rhs: &Self) -> bool {
        (self.bits & rhs.bits) == self.bits
    }

    /// Checks if there is a non-empty intersection between this and `rhs`.
    #[must_use]
    pub fn overlaps(&self, rhs: &Self) -> bool {
        (*self & *rhs).any()
    }

    /// Checks if this character class is within the ASCII range.
    #[must_use]
    pub fn is_ascii(&self) -> bool {
        (*self & !Self::from_range(0x00, 0x7f)).none()
    }

    /// Check is this character class represents the first bytes of multi-byte
    /// UTF-8 characters.
    #[must_use]
    pub fn is_utf8_start(&self) -> bool {
        (*self & Self::from_range(0x00, UTF_CONT_MAX)).none()
    }
}

impl fmt::Display for CharReach {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut next = self.find_first();
        while let Some(pos) = next {
            s.push(pos as char);
            next = self.find_next(pos);
        }
        write!(f, "{}", s)
    }
}

impl BitAnd for CharReach {
    type Output = Self;

    #[inline]
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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

/// Fills a bitvector with the contents of the given `CharReach`.
///
/// # Panics
///
/// Panics if `bits` has less than 32 elements.
pub fn fill_bitvector(cr: &CharReach, bits: &mut [u8]) {
    for b in bits[0..32].iter_mut() {
        *b = 0;
    }
    let mut next = cr.find_first();
    while let Some(pos) = next {
        bits[(pos / 8) as usize] |= 1 << (pos & 8);
        next = cr.find_next(pos);
    }
}

/// Generates and- and compare-masks for checking the char reach.
///
/// Any character c in `cr` will satisfy `(c & and_mask) == cmp_mask`, where
/// `(and_mask, cmp_mask)` is the return value of this function.
///
/// Note: characters not in `cr` may also pass the and/cmp checks.
#[must_use]
pub fn make_and_cmp_mask(cr: &CharReach) -> (u8, u8) {
    let mut lo = 0xff;
    let mut hi = 0x00;

    let mut next = cr.find_first();
    while let Some(pos) = next {
        hi |= pos;
        lo &= pos;
        next = cr.find_next(pos);
    }

    let and_mask = !(lo ^ hi);
    let cmp_mask = lo;
    (and_mask, cmp_mask)
}

#[cfg(test)]
mod tests {
    use super::CharReach;
    use std::convert::TryInto;

    #[test]
    fn init() {
        let cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(cr.none());
        assert!(!cr.all());
    }

    #[test]
    fn dot() {
        let dot = CharReach::dot();
        assert_eq!(dot.count(), 256);
        assert!(dot.all());
        for i in 0..=255 {
            assert!(dot.test(i));
        }
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

    #[test]
    fn bytes() {
        let mut cr = CharReach::default();

        cr.set(1);
        cr.set(2);
        cr.set(b'a');
        cr.set(b'Z');
        cr.set(b'm');
        cr.set(b'~');
        cr.set(210);
        assert!(!cr.is_alphabetic());
        cr.flip(1);
        cr.flip(2);
        cr.flip(b'~');
        cr.flip(210);
        assert!(cr.is_alphabetic());

        assert_eq!("Zam", cr.to_string());
    }

    #[test]
    fn alpha() {
        let mut cr = CharReach::default();

        assert_eq!(cr.count(), 0);
        assert!(!cr.is_alphabetic());
        cr.set(b'a');
        assert!(cr.count() != 0);
        assert!(cr.is_alphabetic());
        cr.set(b'A');
        cr.set(b'b');
        cr.set(b'z');
        assert!(cr.is_alphabetic());
        cr.set(1);
        assert!(!cr.is_alphabetic());
    }

    #[test]
    fn alpha_all() {
        for i in 0..=255 {
            let cr = CharReach::from_char(i);
            assert_eq!(
                cr.is_alphabetic(),
                (b'A' <= i && i <= b'Z') || (b'a' <= i && i <= b'z')
            );
        }

        assert!(!CharReach::dot().is_alphabetic());
        assert!(!CharReach::from_bytes(b"0123456789").is_alphabetic());
        assert!(!CharReach::from_bytes(b"a0").is_alphabetic());
        assert!(!CharReach::from_bytes(b"abcdef0").is_alphabetic());
        assert!(!CharReach::from_range(b'A', b'z').is_alphabetic());
        assert!(!CharReach::from_range(b'A', b'Z' + 1).is_alphabetic());
        assert!(!CharReach::from_range(b'A' - 1, b'Z').is_alphabetic());
        assert!(!CharReach::from_range(b'a', b'z' + 1).is_alphabetic());
        assert!(!CharReach::from_range(b'a' - 1, b'z').is_alphabetic());

        assert!(CharReach::from_range(b'A', b'B').is_alphabetic());
        assert!(CharReach::from_range(b'A', b'F').is_alphabetic());
        assert!(CharReach::from_range(b'A', b'Z').is_alphabetic());
        assert!(CharReach::from_range(b'X', b'Z').is_alphabetic());
        assert!(CharReach::from_range(b'a', b'b').is_alphabetic());
        assert!(CharReach::from_range(b'a', b'z').is_alphabetic());
        assert!(CharReach::from_bytes(b"ABCDEFabcdef").is_alphabetic());
    }

    #[test]
    fn caseless() {
        let mut cr = CharReach::default();

        cr.set(b'a');
        assert!(!cr.is_caseless_char());
        cr.set(b'A');
        assert!(cr.is_caseless_char());
        cr.set(b'b');
        assert!(!cr.is_caseless_char());
        cr.set(b'B');
        assert!(!cr.is_caseless_char());
    }

    #[test]
    fn caseless_all() {
        for i in 0..=255 {
            assert!(!CharReach::from_char(i).is_caseless_char());
            for j in 0..=255 {
                let mut cr = CharReach::default();
                cr.set(i);
                cr.set(j);

                let upper_lower = (i >= b'A' && i <= b'Z') && j == i ^ 0x20;
                let lower_upper = (i >= b'a' && i <= b'z') && j == i ^ 0x20;
                let caseless_pair = upper_lower | lower_upper;

                assert_eq!(cr.is_caseless_char(), caseless_pair);
            }
        }
    }

    #[test]
    fn bitwise() {
        let mut cr1 = CharReach::default();
        let mut cr2 = CharReach::default();
        let mut cr3 = CharReach::default();
        let mut cr4 = CharReach::default();

        cr1.set(b'a');
        cr2.set(b'z');
        cr3.set(b'a');
        cr3.set(b'z');
        assert!(cr1 < cr3);

        cr4 |= cr1;
        cr4 |= cr2;
        assert!(cr3 == cr4);
        assert!((cr1 | cr2) == cr3);
        assert!((cr1 | cr2) == cr4);
        assert!((cr1 & cr3) == cr1);
        assert!((cr2 & cr3) == cr2);

        cr3 &= cr1;
        assert!(!cr3.test(b'z'));
    }

    #[test]
    fn bit5() {
        let mut cr = CharReach::default();

        assert!(cr.is_bit5_insensitive());
        cr.set(b'a');
        assert!(!cr.is_bit5_insensitive());
        cr.set(b'A');
        assert!(cr.is_bit5_insensitive());
        cr.set(b'!');
        assert!(!cr.is_bit5_insensitive());
        cr.set(1);
        assert!(cr.is_bit5_insensitive());

        cr.reset_all();
        cr.set(b'!');
        cr.set(b'A');
        assert!(!cr.is_bit5_insensitive());

        cr.reset_all();
        cr.set(b'A');
        cr.set(b'b');
        assert!(!cr.is_bit5_insensitive());
        cr.set(b'a');
        cr.set(b'B');
        assert!(cr.is_bit5_insensitive());
    }

    #[test]
    fn set_range() {
        for range in 0..=255 {
            for from in 0..=255 - range {
                let to = from + range;
                let mut cr = CharReach::default();
                cr.set_range(from, to);
                assert_eq!(cr.find_first(), Some(from));
                assert_eq!(cr.find_last(), Some(to));
                assert_eq!(cr.count(), range as u32 + 1);
            }
        }
    }

    #[test]
    fn find_empty() {
        assert_eq!(CharReach::default().find_first(), None);
        assert_eq!(CharReach::default().find_next(u8::max_value()), None);
    }

    #[test]
    fn find_last() {
        let mut cr = CharReach::default();
        cr.set(b'a');
        assert_eq!(cr.find_last(), Some(b'a'));
        cr.set(b'b');
        assert_eq!(cr.find_last(), Some(b'b'));
        cr.set(192);
        assert_eq!(cr.find_last(), Some(192));
        cr.set(207);
        assert_eq!(cr.find_last(), Some(207));
        cr.set(223);
        assert_eq!(cr.find_last(), Some(223));
        cr.set(255);
        assert_eq!(cr.find_last(), Some(255));

        cr.reset_all();
        assert_eq!(cr.find_last(), None);
        cr.set(0);
        assert_eq!(cr.find_last(), Some(0));
        cr.set(1);
        assert_eq!(cr.find_last(), Some(1));
    }

    #[test]
    fn find_nth() {
        // One bit cases.
        for i in 0..=255 {
            let cr = CharReach::from_char(i);
            assert_eq!(cr.find_nth(0), Some(i));
            assert_eq!(cr.find_nth(1), None);
        }

        // All bits set.
        let dot = CharReach::dot();
        for i in 0..=255 {
            assert_eq!(dot.find_nth(i), Some(i));
        }

        // Trivial two bit cases.
        for i in 0..128 {
            let mut cr = CharReach::default();
            cr.set(i);
            cr.set(255 - i);
            assert_eq!(cr.find_nth(0), Some(i));
            assert_eq!(cr.find_nth(1), Some(255 - i));
            assert_eq!(cr.find_nth(2), None);
        }

        // More complex case.
        const BYTES: &[u8; 44] = b"\x01\x02\x03\x05\x06\x20!#$%&./0123568:;ABCDEFMNOPUYZbcdefwxyz";
        let cr = CharReach::from_bytes(BYTES);
        for i in 0..BYTES.len() {
            assert_eq!(cr.find_nth(i.try_into().unwrap()), Some(BYTES[i]));
        }
        assert_eq!(cr.find_nth(BYTES.len().try_into().unwrap()), None);
    }
}
