#[derive(Clone, Hash, PartialEq, Eq)]
pub(crate) struct Report {}

pub(crate) fn make_e_callback(
    _report: u32,
    _offset_adjust: i32,
    _ekey: Option<u32>,
    _quiet: bool,
) -> Report {
    Report {}
}
