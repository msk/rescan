use crate::Flags;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ParseMode {
    pub(crate) caseless: bool,
    pub(crate) dotall: bool,
    pub(crate) ignore_space: bool,
    pub(crate) multiline: bool,
    pub(crate) ucp: bool,
    pub(crate) utf8: bool,
}

impl ParseMode {
    pub(crate) fn new(flags: Flags) -> Self {
        Self {
            caseless: flags.contains(Flags::CASELESS),
            dotall: flags.contains(Flags::DOTALL),
            ignore_space: false,
            multiline: flags.contains(Flags::MULTILINE),
            ucp: flags.contains(Flags::UCP),
            utf8: flags.contains(Flags::UTF8),
        }
    }
}
