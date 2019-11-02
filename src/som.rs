//! Common SOM definitions.

/// A start of match behavior.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SomType {
    /// No SOM required
    None,

    /// Exact leftmost SOM
    #[allow(dead_code)]
    Left,
}
