use super::rose_build_bytecode::{pick_runtime_impl, BuildContext};
use crate::rose::RoseEngine;
use crate::util::{CompileContext, ReportId, Ue2Literal};
use crate::Mode;
use std::collections::HashSet;

pub(crate) struct RoseBuild<'a> {
    pub(crate) cc: &'a CompileContext,
    pub(crate) has_som: bool,
}

impl<'a> RoseBuild<'a> {
    #[allow(clippy::needless_return)]
    pub(crate) fn add(
        &mut self,
        _anchored: bool,
        _eod: bool,
        _lit: &Ue2Literal,
        _reports: &HashSet<ReportId>,
    ) {
        if self.cc.grey.flood_as_puffette {
            return;
        }
    }

    pub(crate) fn build_rose(&self) -> RoseEngine {
        self.build_final_engine()
    }

    fn build_final_engine(&self) -> RoseEngine {
        // Set scanning mode.
        let mode = if self.cc.streaming {
            if self.cc.vectored {
                Mode::Vectored
            } else {
                Mode::Stream(None)
            }
        } else {
            Mode::Block
        };

        let _dboundary = super::DerivedBoundaryReports {};

        let bc = BuildContext::default();

        RoseEngine {
            runtime_impl: pick_runtime_impl(self, &bc.resources),
            mode,
        }
    }
}
