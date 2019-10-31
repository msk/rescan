use crate::nfagraph::NgHolder;
use crate::rose::RoseBuild;
use crate::util::CompileContext;
use rescan_util::{mixed_sensitivity, Ue2Literal};

pub(crate) struct Ng {
    pub(crate) cc: CompileContext,
    pub(crate) rose: RoseBuild,
}

impl Ng {
    pub(crate) fn new(cc: CompileContext) -> Self {
        Self {
            cc,
            rose: RoseBuild {},
        }
    }

    pub(crate) fn add_graph(&self, _g: NgHolder) {}

    pub(crate) fn add_literal(&self, literal: &Ue2Literal) -> bool {
        debug_assert!(!literal.is_empty());

        if mixed_sensitivity(literal) {
            return false;
        }

        false
    }
}
