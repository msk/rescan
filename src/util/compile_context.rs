use crate::Grey;

/// Structure for describing the compile environment: grey box settings, target
/// arch, mode flags, etc.
pub(crate) struct CompileContext {
    #[allow(dead_code)]
    pub(crate) streaming: bool,
    #[allow(dead_code)]
    pub(crate) vectored: bool,

    pub(crate) grey: Grey,
}

impl CompileContext {
    pub(crate) fn new(streaming: bool, vectored: bool, grey: &Grey) -> Self {
        Self {
            streaming,
            vectored,
            grey: grey.clone(),
        }
    }
}
