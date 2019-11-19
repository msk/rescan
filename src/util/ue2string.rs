use super::compare::{mytoupper, ourisalpha};
use bit_vec::BitVec;
use std::cmp::Ordering;

#[derive(Default, Eq)]
pub struct Ue2Literal {
    s: Vec<u8>,
    nocase: BitVec,
}

pub struct Ue2LiteralElem {
    pub c: u8,
    nocase: bool,
}

pub struct Ue2LiteralIter<'a> {
    lit: &'a Ue2Literal,
    idx: usize,
}

impl Ue2Literal {
    #[must_use]
    pub fn len(&self) -> usize {
        self.s.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    #[must_use]
    pub fn any_nocase(&self) -> bool {
        self.nocase.any()
    }

    #[must_use]
    pub fn iter(&self) -> Ue2LiteralIter {
        Ue2LiteralIter { lit: self, idx: 0 }
    }

    pub fn push(&mut self, mut c: u8, nc: bool) {
        if nc {
            c = mytoupper(c);
        }
        self.nocase.push(nc);
        self.s.push(c);
    }
}

impl<'a> Iterator for Ue2LiteralIter<'a> {
    type Item = Ue2LiteralElem;

    fn next(&mut self) -> Option<Self::Item> {
        let c = *self.lit.s.get(self.idx)?;
        let nocase = self.lit.nocase.get(self.idx)?;
        self.idx += 1;
        Some(Ue2LiteralElem { c, nocase })
    }
}

impl<'a> IntoIterator for &'a Ue2Literal {
    type Item = Ue2LiteralElem;
    type IntoIter = Ue2LiteralIter<'a>;

    #[must_use]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl PartialEq for Ue2Literal {
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.s == other.s
    }
}

impl Ord for Ue2Literal {
    #[must_use]
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.s.cmp(&other.s);
        if ordering == Ordering::Equal {
            self.nocase.cmp(&other.nocase)
        } else {
            ordering
        }
    }
}

impl PartialOrd for Ue2Literal {
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns `true` iff the range of a literal given cannot be considered
/// entirely case-sensitive nor entirely case-insensitive.
pub fn mixed_sensitivity<T>(it: T) -> bool
where
    T: IntoIterator<Item = Ue2LiteralElem>,
{
    let mut cs = false;
    let mut nc = false;
    for e in it {
        if !ourisalpha(e.c) {
            continue;
        }
        if e.nocase {
            nc = true;
        } else {
            cs = true;
        }
    }

    cs && nc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_eq() {
        let empty1 = Ue2Literal::default();
        let empty2 = Ue2Literal::default();

        assert!(empty1 == empty2);
        assert!(!(empty1 != empty2));
    }
}
