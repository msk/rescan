use crate::Flags;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ParseMode {
    pub(crate) caseless: bool,
    pub(crate) ucp: bool,
    pub(crate) utf8: bool,
}

impl ParseMode {
    pub(crate) fn new(flags: Flags) -> Self {
        Self {
            caseless: flags.contains(Flags::CASELESS),
            ucp: flags.contains(Flags::UCP),
            utf8: flags.contains(Flags::UTF8),
        }
    }
}
