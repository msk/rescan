/// Structure tracking which resources are used by this Rose instance at
/// runtime.
///
/// We use this to control how much initialisation we need to do at the
/// beginning of a stream/block at runtime.
#[derive(Default)]
pub(super) struct RoseResources {
    pub(super) has_outfixes: bool,
    pub(super) has_suffixes: bool,
    pub(super) has_leftfixes: bool,
    #[allow(dead_code)]
    pub(super) has_literals: bool,
    pub(super) has_states: bool,
    pub(super) checks_groups: bool,
    pub(super) has_lit_delay: bool,
    pub(super) has_lit_check: bool, // long literal support
    pub(super) has_anchored: bool,
    #[allow(dead_code)]
    pub(super) has_anchored_multiple: bool, // multiple anchored dfas
    #[allow(dead_code)]
    pub(super) has_anchored_large: bool, // mcclellan 16 anchored dfa
    pub(super) has_floating: bool,
    pub(super) has_eod: bool,
}
