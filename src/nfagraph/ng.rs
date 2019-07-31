use crate::util::{mixed_sensitivity, Ue2Literal};

pub(crate) struct Ng {}

impl Ng {
    pub(crate) fn new() -> Self {
        Ng {}
    }

    pub(crate) fn add_literal(&self, literal: &Ue2Literal) -> bool {
        debug_assert!(!literal.is_empty());

        if mixed_sensitivity(literal) {
            return false;
        }

        false
    }
}
