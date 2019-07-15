use bit_vec::BitVec;
use std::cmp::Ordering;

use crate::util::mytoupper;

#[derive(Default, Eq)]
pub(crate) struct Ue2Literal {
    s: Vec<u8>,
    nocase: BitVec,
    pure: bool, // born from cutting or not (pure literal).
}

impl Ue2Literal {
    pub(crate) fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub(crate) fn push(&mut self, mut c: u8, nc: bool) {
        if nc {
            c = mytoupper(c);
        }
        self.nocase.push(nc);
        self.s.push(c);
    }

    pub(crate) fn set_pure(&mut self) {
        self.pure = true;
    }
}

impl PartialEq for Ue2Literal {
    fn eq(&self, other: &Ue2Literal) -> bool {
        self.s == other.s
    }
}

impl Ord for Ue2Literal {
    fn cmp(&self, other: &Ue2Literal) -> Ordering {
        let ordering = self.s.cmp(&other.s);
        if ordering == Ordering::Equal {
            self.nocase.cmp(&other.nocase)
        } else {
            ordering
        }
    }
}

impl PartialOrd for Ue2Literal {
    fn partial_cmp(&self, other: &Ue2Literal) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
