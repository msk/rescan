use crate::compiler::ExpressionInfo;
use crate::nfagraph::NgHolder;
use crate::rose::RoseBuild;
use crate::util::{
    make_e_callback, make_som_relative_callback, BoundaryReports, CompileContext, Depth,
    ExternalReportInfo, ReportManager,
};
use crate::util::{mixed_sensitivity, Ue2Literal};
use crate::{CompileError, ErrorKind, SmallWriteBuild, SomType};
use maplit::hashset;
use std::cmp::min;
use std::convert::TryInto;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

pub(crate) struct Ng<'a> {
    /// The length of the shortest corpus which can match a pattern contained in
    /// the `Ng` (excluding the boundary reports used by vacuous patterns, which
    /// give an effective `min_width` of zero).
    min_width: Depth,

    rm: ReportManager<'a>,
    #[allow(dead_code)]
    boundary: BoundaryReports,
    pub(crate) cc: &'a CompileContext,

    smwr: SmallWriteBuild<'a>,
    pub(crate) rose: RoseBuild<'a>,
    _pin: PhantomPinned,
}

impl<'a> Ng<'a> {
    pub(crate) fn new(
        cc: &'a CompileContext,
        num_patterns: usize,
        _som_precision: usize,
    ) -> Pin<Box<Self>> {
        let res = Self {
            min_width: Depth::infinity(),
            rm: ReportManager::new(&cc.grey),
            boundary: BoundaryReports::default(),
            cc: &cc,
            smwr: SmallWriteBuild::new(num_patterns, cc),
            rose: RoseBuild {
                cc: &cc,
                has_som: false,
            },
            _pin: PhantomPinned,
        };
        let mut boxed = Box::pin(res);

        let rm = NonNull::from(&boxed.rm);
        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).smwr.rm = rm;
        }
        boxed
    }

    pub(crate) fn add_graph(&mut self, expr: &ExpressionInfo, _g: &NgHolder) {
        if expr.som != SomType::None {
            self.rose.has_som = true;
        }
    }

    pub(crate) fn add_literal(
        &mut self,
        literal: &Ue2Literal,
        expr_index: u32,
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

        let id = if let SomType::None = som {
            let ekey = if highlander {
                Some(self.rm.get_exhaustible_key(external_report))
            } else {
                None
            };
            let r = make_e_callback(external_report, 0, ekey, quiet);
            self.rm.get_internal_id(&r)
        } else {
            debug_assert!(!highlander); // not allowed, checked earlier.
            let r = make_som_relative_callback(
                external_report,
                0,
                literal.len().try_into().expect("literal too long"),
            );
            let id = self.rm.get_internal_id(&r);
            self.rose.has_som = true;
            id
        }?;

        self.rose.add(false, false, literal, &hashset! {id});

        self.min_width = min(
            self.min_width,
            literal
                .len()
                .try_into()
                .map_err(|_| CompileError::new(ErrorKind::Other, "depth overflow"))?,
        );

        // Inform small write handler about this literal.
        self.smwr.add_literal(literal, id);

        Ok(true)
    }
}
