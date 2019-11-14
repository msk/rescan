use crate::Mode;

pub(crate) enum RoseRuntimeImpl {
    FullRose,
    PureLiteral,
    SingleOutfix,
}

pub(crate) struct RoseEngine {
    /// Can we just run the floating table or a single outfix or do we need a
    /// full rose?
    pub(crate) runtime_impl: RoseRuntimeImpl,
    /// Scanning mode.
    #[allow(dead_code)]
    pub(super) mode: Mode,
}
