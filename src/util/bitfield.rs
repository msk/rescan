use std::mem::size_of;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

type BlockType = u64;

const BLOCK_SIZE: usize = size_of::<BlockType>() * 8;

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
