#[derive(Clone)]
pub(crate) struct Grey {
    pub(crate) allow_literal: bool,

    pub(crate) shortcut_literals: bool,

    pub(crate) limit_pattern_count: usize,
    pub(crate) limit_pattern_length: usize,
    pub(crate) limit_report_count: usize,
}

impl Default for Grey {
    fn default() -> Self {
        Self {
            allow_literal: true,
            shortcut_literals: true,
            limit_pattern_count: 8_000_000,
            limit_pattern_length: 16_000,
            limit_report_count: 4 * 8_000_000,
        }
    }
}
