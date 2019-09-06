use std::ops::{Add, BitAnd};

/// Report identifier.
///
/// Used for internal IDs and external IDs (those reported on match).
pub type ReportId = u32;

/// Aligns to the next 16-byte boundary.
#[inline]
pub fn round_up_16<T>(a: T) -> T
where
    T: Add + BitAnd + From<u8>,
    <T as Add>::Output: Into<T>,
    <T as BitAnd>::Output: Into<T>,
{
    ((a + 0xf_u8.into()).into() & (!0xf_u8).into()).into()
}

/// Aligns to the previous 16-byte boundary.
#[inline]
pub fn round_down_16<T>(a: T) -> T
where
    T: BitAnd + From<u8>,
    <T as BitAnd>::Output: Into<T>,
{
    (a & (!0xf_u8).into()).into()
}

/// Aligns to the next n-byte boundary.
#[inline]
pub fn round_up_n<T>(a: T, n: usize) -> T
where
    T: Add + BitAnd + From<usize>,
    <T as Add>::Output: Into<T>,
    <T as BitAnd>::Output: Into<T>,
{
    ((a + (n - 1).into()).into() & (!(n - 1)).into()).into()
}

/// Aligns to the previous n-byte boundary.
#[inline]
pub fn round_down_n<T>(a: T, n: usize) -> T
where
    T: BitAnd + From<usize>,
    <T as BitAnd>::Output: Into<T>,
{
    (a & (!(n - 1)).into()).into()
}

/// Aligns to a cache line.
#[inline]
pub fn round_up_cache_line<T>(a: T) -> T
where
    T: Add + BitAnd + From<u8>,
    <T as Add>::Output: Into<T>,
    <T as BitAnd>::Output: Into<T>,
{
    ((a + 0x3f_u8.into()).into() & (!0x3f_u8).into()).into()
}
