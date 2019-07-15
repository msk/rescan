use crate::util::Ue2Literal;

pub(crate) struct Ng {}

impl Ng {
    pub(crate) fn new() -> Self {
        Ng {}
    }

    pub(crate) fn add_literal(&self, literal: &Ue2Literal) -> bool {
        debug_assert!(!literal.is_empty());

        false
    }
}
