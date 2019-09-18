use crate::{CompileError, Grey};
use rescan_util::ReportId;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Clone, Copy)]
pub(crate) struct ExternalReportInfo {
    pub(crate) highlander: bool,
    pub(crate) first_pattern_index: usize,
}

impl ExternalReportInfo {
    pub(crate) fn new(h: bool, fpi: usize) -> Self {
        Self {
            highlander: h,
            first_pattern_index: fpi,
        }
    }
}

/// Tracks Report structures, exhaustion and dedupe keys.
pub(crate) struct ReportManager<'a> {
    /// Grey box ref, for checking resource limits.
    _grey: &'a Grey,

    /// Mapping from external match ids to information about that id.
    external_id_map: HashMap<ReportId, ExternalReportInfo>,

    /// Whether database is globally exhaustible (all patterns must b highlander
    /// for this to be `true`).
    global_exhaust: bool,
}

impl<'a> ReportManager<'a> {
    pub(crate) fn new(g: &'a Grey) -> Self {
        Self {
            _grey: g,
            external_id_map: HashMap::default(),
            global_exhaust: true,
        }
    }

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
}
