use std::mem::size_of;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

type BlockType = u64;

const BLOCK_SIZE: usize = size_of::<BlockType>() * 8;
const ALL_ONES: BlockType = !0;

#[derive(Default)]
pub(crate) struct BitField256 {
    bits: [BlockType; 256 / BLOCK_SIZE],
}

impl BitField256 {
    /// Sets bit N.
    pub(crate) fn set(&mut self, n: u8) {
        self.bits[BitField256::get_word(n)] |= BitField256::mask_bit(n);
    }

    /// Tests bit N.
    pub(crate) fn test(&self, n: u8) -> bool {
        (self.bits[BitField256::get_word(n)] & BitField256::mask_bit(n)) != 0
    }

    /// Returns number of bits set on.
    pub(crate) fn count(&self) -> usize {
        let mut sum = 0;
        sum += self.bits[0].count_ones();
        sum += self.bits[1].count_ones();
        sum += self.bits[2].count_ones();
        sum += self.bits[3].count_ones();
        sum as usize
    }

    /// Returns first bit set.
    pub(crate) fn find_first(&self) -> Option<u8> {
        for (i, &bits) in self.bits.iter().enumerate() {
            if bits != 0 {
                return Some((i * BLOCK_SIZE) as u8 + bits.trailing_zeros() as u8);
            }
        }
        None
    }

    /// Returns next bit set.
    pub(crate) fn find_next(&self, last: u8) -> Option<u8> {
        let last_i = BitField256::get_word(last);
        let mut last_word = self.bits[last_i];

        if last % BLOCK_SIZE as u8 != BLOCK_SIZE as u8 - 1 {
            last_word &= ALL_ONES << ((last % BLOCK_SIZE as u8) + 1);

            if last_word != 0 {
                return Some((last_i * BLOCK_SIZE) as u8 + last_word.trailing_zeros() as u8);
            }
        }

        for (i, &bits) in self.bits.iter().enumerate() {
            if bits != 0 {
                return Some(((last_i + i) * BLOCK_SIZE) as u8 + bits.trailing_zeros() as u8);
            }
        }

        None
    }

    fn get_word(n: u8) -> usize {
        (n as usize) / BLOCK_SIZE
    }

    fn mask_bit(n: u8) -> BlockType {
        (1 as BlockType) << ((n as usize) % BLOCK_SIZE)
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

#[cfg(test)]
mod test {
    use super::BitField256;

    #[test]
    fn bitfield_empty() {
        let a = BitField256::default();
        assert_eq!(a.count(), 0);
    }
}
