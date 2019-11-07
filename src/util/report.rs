#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) struct Report {
    /// Minimum match length (start of match to current offset).
    pub(crate) min_length: u64,
}

pub(crate) fn make_e_callback(
    _report: u32,
    _offset_adjust: i32,
    _ekey: Option<u32>,
    _quiet: bool,
) -> Report {
    Report { min_length: 0 }
}

pub(crate) fn make_som_relative_callback(
    _report: u32,
    _offset_adjust: i32,
    _distance: u64,
) -> Report {
    Report { min_length: 0 }
}
