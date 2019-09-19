use std::cmp::Ordering;
use std::convert::TryFrom;

const INFINITY: u32 = (1 << 31) - 1;
#[allow(dead_code)]
const UNREACHABLE: u32 = 1 << 31;
const MAX_VALUE: u32 = INFINITY - 1;

pub(crate) struct DepthOverflowError {}

/// Type used to represent depth information; value is either a count, or the
/// special values "infinity" and "unreachable".
#[derive(Clone, Copy, Debug, Eq)]
pub(crate) struct Depth {
    val: u32,
}

impl Depth {
    pub(crate) fn infinity() -> Self {
        Self { val: INFINITY }
    }
}

impl TryFrom<u32> for Depth {
    type Error = DepthOverflowError;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        if val <= MAX_VALUE {
            Ok(Self { val })
        } else {
            Err(DepthOverflowError {})
        }
    }
}

impl TryFrom<usize> for Depth {
    type Error = DepthOverflowError;

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        if val <= MAX_VALUE as usize {
            #[allow(clippy::cast_possible_truncation)]
            Ok(Self { val: val as u32 })
        } else {
            Err(DepthOverflowError {})
        }
    }
}

impl PartialEq for Depth {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
