#[derive(Clone, Copy, Default)]
pub(crate) struct ParseMode {
    pub(crate) caseless: bool,
    pub(crate) ucp: bool,
    pub(crate) utf8: bool,
}
