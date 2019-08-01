use crate::util::{mixed_sensitivity, CompileContext, Ue2Literal};

pub(crate) struct Ng {
    pub(crate) cc: CompileContext,
}

impl Ng {
    pub(crate) fn new(cc: CompileContext) -> Self {
        Ng { cc }
    }

    pub(crate) fn add_literal(&self, literal: &Ue2Literal) -> bool {
        debug_assert!(!literal.is_empty());

        if mixed_sensitivity(literal) {
            return false;
        }

        false
    }
}
