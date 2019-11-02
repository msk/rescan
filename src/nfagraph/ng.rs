use crate::compiler::ExpressionInfo;
use crate::nfagraph::NgHolder;
use crate::rose::RoseBuild;
use crate::util::CompileContext;
use crate::SomType;
use rescan_util::{mixed_sensitivity, Ue2Literal};

pub(crate) struct Ng {
    pub(crate) cc: CompileContext,
    pub(crate) rose: RoseBuild,
}

impl Ng {
    pub(crate) fn new(cc: CompileContext, _num_patterns: usize, _som_precision: usize) -> Self {
        Self {
            cc,
            rose: RoseBuild { has_som: false },
        }
    }

    pub(crate) fn add_graph(&mut self, expr: &ExpressionInfo, _g: &NgHolder) {
        if expr.som != SomType::None {
            self.rose.has_som = true;
        }
    }

    pub(crate) fn add_literal(&self, literal: &Ue2Literal) -> bool {
        debug_assert!(!literal.is_empty());

        if mixed_sensitivity(literal) {
            return false;
        }

        false
    }
}
