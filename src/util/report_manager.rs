use crate::util::Report;
use crate::{CompileError, ErrorKind, Grey};
use rescan_util::{ReportId, S64a};
use std::collections::{hash_map::Entry, BTreeMap, HashMap};
use std::convert::TryInto;

#[derive(Clone, Copy)]
pub(crate) struct ExternalReportInfo {
    pub(crate) highlander: bool,
    pub(crate) first_pattern_index: u32,
}

impl ExternalReportInfo {
    pub(crate) fn new(h: bool, fpi: u32) -> Self {
        Self {
            highlander: h,
            first_pattern_index: fpi,
        }
    }
}

/// Tracks Report structures, exhaustion and dedupe keys.
pub(crate) struct ReportManager<'a> {
    /// Grey box ref, for checking resource limits.
    grey: &'a Grey,

    /// Report structures, indexed by ID.
    report_ids: Vec<Report>,

    /// Mapping from Report to ID (inverse of `report_ids` vector).
    report_id_to_internal_map: HashMap<Report, u32>,

    /// Mapping from external match ids to information about that id.
    external_id_map: HashMap<ReportId, ExternalReportInfo>,

    /// Mapping from expression index to exhaustion key.
    to_exhaustible_key_map: BTreeMap<S64a, u32>,

    /// Whether database is globally exhaustible (all patterns must b highlander
    /// for this to be `true`).
    global_exhaust: bool,
}

impl<'a> ReportManager<'a> {
    pub(crate) fn new(g: &'a Grey) -> Self {
        Self {
            grey: g,
            report_ids: Vec::default(),
            report_id_to_internal_map: HashMap::default(),
            external_id_map: HashMap::default(),
            to_exhaustible_key_map: BTreeMap::default(),
            global_exhaust: true,
        }
    }

    /// Fetches the ID associated with the given Report.
    pub(crate) fn get_internal_id(&mut self, ir: &Report) -> Result<u32, CompileError> {
        if let Some(id) = self.report_id_to_internal_map.get(ir) {
            return Ok(*id);
        }

        // Construct a new internal report and assign it a ReportID.

        if self.num_reports() >= self.grey.limit_report_count {
            return Err(CompileError::new(
                ErrorKind::ResourceLimit,
                "Resource limit exceeded.",
            ));
        }

        let size = self.report_ids.len().try_into().expect("too many reports");
        self.report_ids.push(ir.clone());
        self.report_id_to_internal_map.insert(ir.clone(), size);
        Ok(size)
    }

    /// Fetches the Report associated with `id`.
    ///
    /// # Panics
    ///
    /// Panics if `id` is out of bound.
    pub(crate) fn get_report(&self, id: u32) -> &Report {
        &self.report_ids[id as usize]
    }

    /// Total number of reports.
    fn num_reports(&self) -> usize {
        self.report_ids.len()
    }

    /// Registers an external report and validate that we are not violating
    /// highlander constraints (which will cause an exception to be thrown).
    pub(crate) fn register_ext_report(
        &mut self,
        id: ReportId,
        ext: ExternalReportInfo,
    ) -> Result<(), CompileError> {
        match self.external_id_map.entry(id) {
            Entry::Occupied(e) => {
                let eri = e.get();
                if eri.highlander != ext.highlander {
                    // We have a problem.
                    let out = format!(
                    "Expression (index {}) with match ID {} {} SINGLEMATCH whereas previous expression (index {}) with the same match ID did{}",
                    ext.first_pattern_index,
                    id,
                    if ext.highlander {
                        "specified "
                    } else {
                        "did not sepcify "
                    },
                    eri.first_pattern_index,
                    if eri.highlander {
                        " not."
                    } else {
                        "."
                    },
                );
                    return Err(CompileError::with_index(ext.first_pattern_index, out));
                }
            }
            Entry::Vacant(e) => {
                e.insert(ext);
            }
        }

        // Any non-highlander pattern will render us not globally exhaustible.
        if !ext.highlander {
            self.global_exhaust = false;
        }
        Ok(())
    }

    /// Fetch the ekey associated with the given expression index, assigning one
    /// if necessary.
    ///
    /// # Panics
    ///
    /// Panics if the expression index is larger than `u32::max_value()`.
    pub(crate) fn get_exhaustible_key(&mut self, a: u32) -> u32 {
        let size: u32 = self
            .to_exhaustible_key_map
            .len()
            .try_into()
            .expect("too many expressions");
        *self.to_exhaustible_key_map.entry(a.into()).or_insert(size)
    }
}
