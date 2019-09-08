use crate::round_up_n;
use std::mem::size_of;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

type BlockType = u64;

const BLOCK_SIZE: u8 = size_of::<BlockType>() as u8 * 8;
const ALL_ONES: BlockType = !0;

/// Bitset class for 256 elements with find_first and find_next operations.
///
/// Note: underlying storage is allocated as an array of 64-bit blocks. All
/// mutating operations MUST ensure that the trailer (the bits between
/// requested_size and the end of the array) is filled with zeroes; there's a
/// clear_trailer member function for this.
#[derive(Debug, Default)]
pub(crate) struct BitField256 {
    bits: [BlockType; 4],
}

impl BitField256 {
    /// Sets all bits.
    pub(crate) fn setall(&mut self) {
        self.bits[0] = ALL_ONES;
        self.bits[1] = ALL_ONES;
        self.bits[2] = ALL_ONES;
        self.bits[3] = ALL_ONES;
    }

    /// Clears all bits.
    pub(crate) fn clear(&mut self) {
        self.bits[0] = 0;
        self.bits[1] = 0;
        self.bits[2] = 0;
        self.bits[3] = 0;
    }

    /// Sets bit N.
    pub(crate) fn set(&mut self, n: u8) {
        self.bits[BitField256::get_word(n) as usize] |= BitField256::mask_bit(n);
    }

    /// Tests bit N.
    pub(crate) fn test(&self, n: u8) -> bool {
        (self.bits[BitField256::get_word(n) as usize] & BitField256::mask_bit(n)) != 0
    }

    /// Switches on the bits in the range `from..=to`.
    pub(crate) fn set_range(&mut self, from: u8, to: u8) {
        debug_assert!(from <= to);

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
            self.bits[i / BLOCK_SIZE as usize] |= Self::mask_bit(to + 1) - 1;
        }
    }

    /// Returns number of bits set on.
    pub(crate) fn count(&self) -> u8 {
        let mut sum = 0;
        sum += self.bits[0].count_ones();
        sum += self.bits[1].count_ones();
        sum += self.bits[2].count_ones();
        sum += self.bits[3].count_ones();
        sum as u8
    }

    /// Returns first bit set.
    pub(crate) fn find_first(&self) -> Option<u8> {
        for (i, &bits) in self.bits.iter().enumerate() {
            if bits != 0 {
                return Some(i as u8 * BLOCK_SIZE + bits.trailing_zeros() as u8);
            }
        }
        None
    }

    /// Returns next bit set.
    pub(crate) fn find_next(&self, last: u8) -> Option<u8> {
        let last_i = BitField256::get_word(last);
        let mut last_word = self.bits[last_i as usize];

        if last % BLOCK_SIZE != BLOCK_SIZE - 1 {
            last_word &= ALL_ONES << (last % BLOCK_SIZE + 1);

            if last_word != 0 {
                return Some(last_i * BLOCK_SIZE + last_word.trailing_zeros() as u8);
            }
        }

        for (i, &bits) in self.bits.iter().enumerate() {
            if bits != 0 {
                return Some((last_i + i as u8) * BLOCK_SIZE + bits.trailing_zeros() as u8);
            }
        }

        None
    }

    pub(crate) fn bitor(&mut self, rhs: &Self) {
        self.bits[0] |= rhs.bits[0];
        self.bits[1] |= rhs.bits[1];
        self.bits[2] |= rhs.bits[2];
        self.bits[3] |= rhs.bits[3];
    }

    fn get_word(n: u8) -> u8 {
        n / BLOCK_SIZE
    }

    fn mask_bit(n: u8) -> BlockType {
        (1 as BlockType) << (n % BLOCK_SIZE)
    }
}

impl BitAnd for BitField256 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs &= self;
        rhs
    }
}

impl BitAndAssign for BitField256 {
    fn bitand_assign(&mut self, a: Self) {
        self.bits[0] &= a.bits[0];
        self.bits[1] &= a.bits[1];
        self.bits[2] &= a.bits[2];
        self.bits[3] &= a.bits[3];
    }
}

impl BitOr for BitField256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs |= self;
        rhs
    }
}

impl BitOrAssign for BitField256 {
    fn bitor_assign(&mut self, a: Self) {
        self.bits[0] |= a.bits[0];
        self.bits[1] |= a.bits[1];
        self.bits[2] |= a.bits[2];
        self.bits[3] |= a.bits[3];
    }
}

impl PartialEq for BitField256 {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

#[cfg(test)]
mod test {
    use super::BitField256;

    #[test]
    fn bitfield_empty() {
        let a = BitField256::default();
        assert_eq!(a.count(), 0);
    }

    #[test]
    fn set_range_one() {
        let mut a = BitField256::default();
        for i in 0..=255 {
            a.clear();
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
        b.setall();

        assert_eq!(a, b);
    }

    #[test]
    fn set_range_part() {
        let mut a = BitField256::default();
        const PART: usize = 256 / 3;

        for i in 0..256 - PART {
            a.clear();
            a.set_range(i as u8, (i + PART) as u8);

            for j in i..=i + PART {
                assert!(a.test(j as u8), format!("bit {} should be on", j));
            }

            // only the set bits should be on.
            assert_eq!(PART + 1, a.count() as usize);
        }
    }
}
