use rescan_util::ReportId;
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct BoundaryReports {
    /// Set of internal reports to fire unconditionally at offset 0.
    #[allow(dead_code)]
    report_at_0: HashSet<ReportId>,
    /// Set of internal reports to fire unconditionally at offset 0 if it is eod.
    #[allow(dead_code)]
    report_at_0_eod: HashSet<ReportId>,
    /// Set of internal reports to fire unconditionally at eod.
    #[allow(dead_code)]
    report_at_eod: HashSet<ReportId>,
}
