use crate::compiler::ExpressionInfo;
use crate::nfagraph::NgHolder;
use crate::rose::RoseBuild;
use crate::util::{make_e_callback, CompileContext, ExternalReportInfo, ReportManager};
use crate::{CompileError, SomType};
use rescan_util::{mixed_sensitivity, Ue2Literal};

pub(crate) struct Ng<'a> {
    rm: ReportManager<'a>,
    pub(crate) cc: &'a CompileContext,

    pub(crate) rose: RoseBuild,
}

impl<'a> Ng<'a> {
    pub(crate) fn new(cc: &'a CompileContext, _num_patterns: usize, _som_precision: usize) -> Self {
        Self {
            rm: ReportManager::new(&cc.grey),
            cc,
            rose: RoseBuild { has_som: false },
        }
    }

    pub(crate) fn add_graph(&mut self, expr: &ExpressionInfo, _g: &NgHolder) {
        if expr.som != SomType::None {
            self.rose.has_som = true;
        }
    }

    pub(crate) fn add_literal(
        &mut self,
        literal: &Ue2Literal,
        expr_index: usize,
        external_report: u32,
        highlander: bool,
        som: SomType,
        quiet: bool,
    ) -> Result<bool, CompileError> {
        debug_assert!(!literal.is_empty());

        if !self.cc.grey.shortcut_literals {
            return Ok(false);
        }

        // We can't natively handle arbitrary literals with mixed case
        // sensitivity in Rose -- they require mechanisms like benefits masks,
        // which have length limits etc. Better to let those go through full
        // graph processing.
        if mixed_sensitivity(literal) {
            return Ok(false);
        }

        // Register external report and validate highlander constraints.
        self.rm.register_ext_report(
            external_report,
            ExternalReportInfo::new(highlander, expr_index),
        )?;

        let _id = if let SomType::None = som {
            let ekey = if highlander {
                Some(self.rm.get_exhaustible_key(external_report))
            } else {
                None
            };
            let r = make_e_callback(external_report, 0, ekey, quiet);
            self.rm.get_internal_id(&r)
        } else {
            debug_assert!(!highlander); // not allowed, checked earlier.
            unimplemented!();
        };

        Ok(false)
    }
}
