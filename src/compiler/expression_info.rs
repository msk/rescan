use crate::util::ReportId;
use crate::SomType;

/// Properties of an expression.
#[derive(Clone)]
pub(crate) struct ExpressionInfo {
    /// Index of the expression represented by this graph.
    ///
    /// Used:
    /// - down the track in error handling
    /// - for identifying parts of an expression in highlander mode
    pub(crate) index: usize,

    /// Report ID specified by the user.
    pub(crate) report: ReportId,

    /// Vacuous pattern is allowed. (HS_FLAG_ALLOWEMPTY)
    #[allow(dead_code)]
    pub(super) allow_vacuous: bool,

    /// "Highlander" (single match) pattern. (HS_FLAG_SINGLEMATCH)
    pub(crate) highlander: bool,

    /// UTF-8 pattern. (`HS_FLAG_UTF8`)
    pub(super) utf8: bool,

    /// Prefiltering pattern. (`HS_FLAG_PREFILTER`)
    pub(super) prefilter: bool,

    /// Start-of-match type requested, or SOM_NONE.
    pub(crate) som: SomType,

    /// Quiet on match.
    pub(crate) quiet: bool,
}
