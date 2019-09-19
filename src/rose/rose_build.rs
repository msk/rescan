use crate::rose::RoseEngine;
use crate::util::CompileContext;
use crate::Mode;
use rescan_util::{ReportId, Ue2Literal};
use std::collections::HashSet;

pub(crate) struct RoseBuild<'a> {
    pub(crate) cc: &'a CompileContext,
    pub(crate) has_som: bool,
}

impl<'a> RoseBuild<'a> {
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

        RoseEngine { mode }
    }
}
