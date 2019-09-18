use crate::{mytoupper, ourisalpha};
use bit_vec::BitVec;
use std::cmp::Ordering;

#[derive(Default, Eq)]
pub struct Ue2Literal {
    s: Vec<u8>,
    nocase: BitVec,
}

struct Ue2LiteralElem {
    c: u8,
    nocase: bool,
}

struct Ue2LiteralIter<'a> {
    lit: &'a Ue2Literal,
    idx: usize,
}

impl Ue2Literal {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    fn iter(&self) -> Ue2LiteralIter {
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

fn mixed_sensitivity_in<T>(it: T) -> bool
where
    T: Iterator<Item = Ue2LiteralElem>,
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

    nc && cs
}

#[must_use]
pub fn mixed_sensitivity(s: &Ue2Literal) -> bool {
    mixed_sensitivity_in(s.iter())
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
