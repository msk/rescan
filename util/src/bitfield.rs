use crate::round_up_n;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

const BLOCK_SIZE: u8 = 64; // size_of::<BlockType>() * 8;
const ALL_ONES: u64 = !0;

/// Bitset class for 256 elements with `find_first` and `find_next` operations.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub(crate) struct BitField256 {
    bits: [u64; 4],
}

impl BitField256 {
    /// Resets all bits.
    pub(crate) fn reset_all(&mut self) {
        self.bits[0] = 0;
        self.bits[1] = 0;
        self.bits[2] = 0;
        self.bits[3] = 0;
    }

    /// Sets all bits.
    pub(crate) fn set_all(&mut self) {
        self.bits[0] = ALL_ONES;
        self.bits[1] = ALL_ONES;
        self.bits[2] = ALL_ONES;
        self.bits[3] = ALL_ONES;
    }

    /// Resets bit `n`.
    pub(crate) fn reset(&mut self, n: u8) {
        self.bits[Self::get_word(n) as usize] &= !Self::mask_bit(n);
    }

    /// Sets bit `n`.
    pub(crate) fn set(&mut self, n: u8) {
        self.bits[Self::get_word(n) as usize] |= Self::mask_bit(n);
    }

    /// Tests bit `n`.
    pub(crate) fn test(&self, n: u8) -> bool {
        (self.bits[Self::get_word(n) as usize] & Self::mask_bit(n)) != 0
    }

    /// Flips all bits.
    pub(crate) fn flip_all(&mut self) {
        self.bits[0] = !self.bits[0];
        self.bits[1] = !self.bits[1];
        self.bits[2] = !self.bits[2];
        self.bits[3] = !self.bits[3];
    }

    /// Flips bit `n`.
    pub(crate) fn flip(&mut self, n: u8) {
        self.bits[Self::get_word(n) as usize] ^= Self::mask_bit(n);
    }

    /// Switches on the bits in the range `from..=to`.
    ///
    /// # Panics
    ///
    /// Panics if `from > to`.
    pub(crate) fn set_range(&mut self, from: u8, to: u8) {
        assert!(from <= to, "`from` is greater than `to`");

        if from / BLOCK_SIZE == to / BLOCK_SIZE {
            // Small case, our indices are in the same block.
            let mut block = ALL_ONES << (from % BLOCK_SIZE);
            if to % BLOCK_SIZE != BLOCK_SIZE - 1 {
                block &= Self::mask_bit(to + 1) - 1;
            }
            self.bits[(from / BLOCK_SIZE) as usize] |= block;
            return;
        }

        // Large case, work in block units. Write a partial mask, then a
        // run of all-ones blocks, then a partial mask at the end.
        let mut i = from as usize;
        if i % BLOCK_SIZE as usize != 0 {
            let block = ALL_ONES << (i % BLOCK_SIZE as usize);
            self.bits[i / BLOCK_SIZE as usize] |= block;
            i = round_up_n(i, BLOCK_SIZE);
        }

        while i + BLOCK_SIZE as usize <= to as usize + 1 {
            self.bits[i / BLOCK_SIZE as usize] = ALL_ONES;
            i += BLOCK_SIZE as usize;
        }

        if i <= to as usize {
            debug_assert!(to as usize - i + 1 < BLOCK_SIZE as usize);
            self.bits[i / BLOCK_SIZE as usize] |= Self::mask_bit(to.wrapping_add(1)) - 1;
        }
    }

    /// Returns number of bits set on.
    pub(crate) fn count(&self) -> u32 {
        let mut sum = 0;
        sum += self.bits[0].count_ones();
        sum += self.bits[1].count_ones();
        sum += self.bits[2].count_ones();
        sum += self.bits[3].count_ones();
        sum
    }

    /// Returns `true` if no bit is set.
    pub(crate) fn none(&self) -> bool {
        self.bits[0] == 0 && self.bits[1] == 0 && self.bits[2] == 0 && self.bits[3] == 0
    }

    /// Retruns `true` if any bit is set.
    pub(crate) fn any(&self) -> bool {
        !self.none()
    }

    /// Returns `true` if all bits are set.
    pub(crate) fn all(&self) -> bool {
        self.bits[0] == ALL_ONES
            && self.bits[1] == ALL_ONES
            && self.bits[2] == ALL_ONES
            && self.bits[3] == ALL_ONES
    }

    /// Returns first bit set.
    pub(crate) fn find_first(&self) -> Option<u8> {
        for (i, &bits) in self.bits.iter().enumerate() {
            if bits != 0 {
                #[allow(clippy::cast_possible_truncation)]
                return Some(i as u8 * BLOCK_SIZE + bits.trailing_zeros() as u8);
            }
        }
        None
    }

    /// Returns last bit set.
    pub(crate) fn find_last(&self) -> Option<u8> {
        for (i, &bits) in self.bits.iter().enumerate().rev() {
            if bits != 0 {
                #[allow(clippy::cast_possible_truncation)]
                return Some(i as u8 * BLOCK_SIZE + (BLOCK_SIZE - 1) - bits.leading_zeros() as u8);
            }
        }
        None
    }

    /// Returns next bit set.
    pub(crate) fn find_next(&self, last: u8) -> Option<u8> {
        let mut last_i = Self::get_word(last);
        let mut last_word = self.bits[last_i as usize];

        if last % BLOCK_SIZE != BLOCK_SIZE - 1 {
            last_word &= ALL_ONES << (last % BLOCK_SIZE + 1);

            if last_word != 0 {
                #[allow(clippy::cast_possible_truncation)]
                return Some(last_i * BLOCK_SIZE + last_word.trailing_zeros() as u8);
            }
        }

        last_i += 1;
        for (i, &bits) in self.bits[last_i as usize..].iter().enumerate() {
            if bits != 0 {
                #[allow(clippy::cast_possible_truncation)]
                return Some((last_i + i as u8) * BLOCK_SIZE + bits.trailing_zeros() as u8);
            }
        }

        None
    }

    /// Returns (zero-based) `n`-th bit set.
    pub(crate) fn find_nth(&self, n: u8) -> Option<u8> {
        let mut sum = 0;
        for (i, &bits) in self.bits.iter().enumerate() {
            let mut block = bits;
            let after_sum = sum + block.count_ones();
            if after_sum > n.into() {
                // block contains the n-th bit.
                for _ in sum..n.into() {
                    debug_assert!(block > 0);
                    block &= block - 1;
                }
                debug_assert!(block > 0);
                #[allow(clippy::cast_possible_truncation)]
                let bit = i as u8 * BLOCK_SIZE + block.trailing_zeros() as u8;
                debug_assert!(self.test(bit));
                return Some(bit);
            }
            sum = after_sum;
        }

        debug_assert!(self.count() <= n.into());
        None
    }

    fn get_word(n: u8) -> u8 {
        n / BLOCK_SIZE
    }

    fn mask_bit(n: u8) -> u64 {
        1_u64 << (n % BLOCK_SIZE)
    }
}

impl BitAnd for BitField256 {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs &= self;
        rhs
    }
}

impl BitAndAssign for BitField256 {
    #[inline]
    fn bitand_assign(&mut self, a: Self) {
        self.bits[0] &= a.bits[0];
        self.bits[1] &= a.bits[1];
        self.bits[2] &= a.bits[2];
        self.bits[3] &= a.bits[3];
    }
}

impl BitOr for BitField256 {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs |= self;
        rhs
    }
}

impl BitOrAssign for BitField256 {
    #[inline]
    fn bitor_assign(&mut self, a: Self) {
        self.bits[0] |= a.bits[0];
        self.bits[1] |= a.bits[1];
        self.bits[2] |= a.bits[2];
        self.bits[3] |= a.bits[3];
    }
}

impl BitXor for BitField256 {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs ^= self;
        rhs
    }
}

impl BitXorAssign for BitField256 {
    #[inline]
    fn bitxor_assign(&mut self, a: Self) {
        self.bits[0] ^= a.bits[0];
        self.bits[1] ^= a.bits[1];
        self.bits[2] ^= a.bits[2];
        self.bits[3] ^= a.bits[3];
    }
}

impl Not for BitField256 {
    type Output = Self;

    #[inline]
    fn not(mut self) -> Self::Output {
        self.bits[0] = !self.bits[0];
        self.bits[1] = !self.bits[1];
        self.bits[2] = !self.bits[2];
        self.bits[3] = !self.bits[3];
        self
    }
}

#[cfg(test)]
mod tests {
    use super::BitField256;
    use std::convert::TryInto;

    #[test]
    fn empty() {
        let a = BitField256::default();
        assert!(a.none());
        assert_eq!(a.count(), 0);
    }

    #[test]
    fn set_1() {
        let mut a = BitField256::default();
        for i in 0..=255 {
            a.reset_all();
            assert!(a.none());

            a.set(i);
            assert!(!a.none());
            assert!(a.test(i));
            assert_eq!(a.find_first(), Some(i));
            assert!(a.find_next(i).is_none());
            assert_eq!(a.count(), 1);

            a.reset(i);
            assert!(a.none());
            assert!(!a.test(i));
            assert!(a.find_first().is_none());
            assert_eq!(a.count(), 0);
        }
    }

    #[test]
    fn set_n() {
        const STRIDES: [usize; 4] = [80, 17, 7, 3];
        for &step in &STRIDES {
            let mut a = BitField256::default();
            let mut num = 0;
            for i in (0..=255).step_by(step) {
                a.set(i);
                num += 1;
            }
            assert_eq!(a.count(), num);
            assert_eq!(a.find_first(), Some(0));

            let mut count = 1;
            let mut last = 0;
            while let Some(i) = a.find_next(last) {
                assert_eq!(i as usize, count * step);
                count += 1;
                last = i;
            }
        }
    }

    #[test]
    fn all_bits() {
        let mut a = BitField256::default();
        a.set_all();
        assert!(!a.none());
        assert!(a.all());
        assert_eq!(a.count(), 256);

        a.flip_all();
        assert!(a.none());
        assert!(!a.all());
        assert_eq!(a.count(), 0);
    }

    #[test]
    fn flip_n() {
        let mut a = BitField256::default();
        a.set_all();
        assert!(!a.none());
        assert!(a.all());
        assert_eq!(a.count(), 256);

        for i in 0..=255 {
            a.flip(i);
            assert!(!a.test(i));
            assert_eq!(a.count(), 255 - i as u32);
        }
    }

    #[test]
    fn flip_on() {
        let mut a = BitField256::default();
        assert!(a.none());

        for i in 0..=255 {
            a.flip(i);
            assert!(a.test(i));
            assert_eq!(i as u32 + 1, a.count());
        }

        assert!(a.all());
    }

    #[test]
    fn trivial_operations() {
        let a = !BitField256::default();
        let b = BitField256::default();
        assert!(a.all());
        assert!(b.none());

        assert_eq!(a, !b);
        assert_eq!(b, !a);

        assert_eq!(a, a | b);
        assert_eq!(b, a & b);

        let mut c = a;
        assert_eq!(c, a);
        c &= b;
        assert_eq!(c, b);
        c |= a;
        assert_eq!(c, a);
        c = a ^ b;
        assert_eq!(c, a);
    }

    #[test]
    fn even_odd() {
        let mut even = BitField256::default();
        let mut odd = BitField256::default();

        for i in 0..=255 {
            if i % 2 == 0 {
                even.set(i);
            } else {
                odd.set(i);
            }
        }

        for i in 0..=255 {
            if i % 2 == 0 {
                assert!(even.test(i));
                assert!(!odd.test(i));
            } else {
                assert!(odd.test(i));
                assert!(!even.test(i));
            }
        }

        assert!(even != odd);

        assert!((even | odd).all());
        assert!((even ^ odd).all());
        assert!((even & odd).none());
    }

    #[test]
    fn find_first() {
        let mut a = BitField256::default();
        a.set_all();
        assert!(a.all());

        for i in 0..=255 {
            assert_eq!(a.find_first(), Some(i));
            a.reset(i);
        }

        assert!(a.none());
        assert!(a.find_first().is_none());
    }

    #[test]
    fn find_last() {
        let mut a = BitField256::default();
        a.set_all();
        assert!(a.all());

        for i in (0..=255).rev() {
            assert_eq!(a.find_last(), Some(i));
            a.reset(i);
        }

        assert!(a.none());
        assert!(a.find_last().is_none());
    }

    #[test]
    fn find_next_all() {
        let mut a = BitField256::default();
        a.set_all();
        assert!(a.all());
        assert_eq!(a.find_first(), Some(0));

        for i in 1..=255 {
            assert_eq!(a.find_next(i - 1), Some(i));
        }

        assert!(a.find_next(255).is_none());
    }

    #[test]
    fn find_next_none() {
        let mut a = BitField256::default();
        for i in 0..=255 {
            a.reset_all();
            a.set(i);
            assert_eq!(a.find_first(), Some(i));
            assert!(a.find_next(i).is_none());
        }
    }

    #[test]
    fn find_next_last() {
        let mut a = BitField256::default();
        for i in 0..255 {
            a.reset_all();
            a.set(i);
            a.set(255);
            assert_eq!(a.find_first(), Some(i));
            assert_eq!(a.find_next(i), Some(255));
        }
    }

    #[test]
    fn find_nth_one() {
        let mut a = BitField256::default();
        for i in 0..=255 {
            a.reset_all();
            a.set(i);
            assert_eq!(a.find_nth(0), Some(i));
            assert!(a.find_nth(1).is_none());
        }
    }

    #[test]
    fn find_nth_all() {
        let mut a = BitField256::default();
        a.set_all();

        for i in 0..=255 {
            assert_eq!(a.find_nth(i), Some(i));
        }
    }

    #[test]
    fn find_nth_sparse() {
        let mut a = BitField256::default();
        const STRIDE: usize = 256 / 31;

        let mut bits = Vec::<u8>::new();
        for i in (0..=255).step_by(STRIDE) {
            a.set(i);
            bits.push(i);
        }

        assert_eq!(a.count() as usize, bits.len());

        for (n, &pos) in bits.iter().enumerate() {
            assert_eq!(
                a.find_nth(n.try_into().expect("should be less than 256")),
                Some(pos)
            );
        }
    }

    #[test]
    fn set_range_one() {
        let mut a = BitField256::default();
        for i in 0..=255 {
            a.reset_all();
            a.set_range(i, i);

            let mut b = BitField256::default();
            b.set(i.into());

            assert_eq!(a, b);
        }
    }

    #[test]
    fn set_range_all() {
        let mut a = BitField256::default();
        a.set_range(0, 255);

        let mut b = BitField256::default();
        b.set_all();

        assert_eq!(a, b);
    }

    #[test]
    fn set_range_part() {
        let mut a = BitField256::default();
        const PART: usize = 256 / 3;

        for i in 0..256 - PART {
            a.reset_all();
            a.set_range(i as u8, (i + PART) as u8);

            for j in i..=i + PART {
                assert!(a.test(j as u8), format!("bit {} should be on", j));
            }

            assert_eq!(
                PART + 1,
                a.count() as usize,
                "only the set bits should be on"
            );
        }
    }
}
