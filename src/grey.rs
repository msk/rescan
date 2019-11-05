#[derive(Clone)]
pub(crate) struct Grey {
    pub(crate) allow_literal: bool,

    pub(crate) flood_as_puffette: bool,

    pub(crate) shortcut_literals: bool,

    // SmallWrite engine
    pub(crate) allow_small_write: bool,
    pub(crate) small_write_largest_buffer: usize,
    pub(crate) small_write_max_patterns: usize,
    pub(crate) small_write_max_literals: usize,

    pub(crate) limit_pattern_count: usize,
    pub(crate) limit_pattern_length: usize,
    pub(crate) limit_report_count: usize,
}

impl Default for Grey {
    fn default() -> Self {
        Self {
            allow_literal: true,
            flood_as_puffette: false,
            shortcut_literals: true,
            allow_small_write: true, // McClellan dfas for small patterns
            small_write_largest_buffer: 70,
            small_write_max_patterns: 10_000,
            small_write_max_literals: 10_000,
            limit_pattern_count: 8_000_000,
            limit_pattern_length: 16_000,
            limit_report_count: 4 * 8_000_000,
        }
    }
}
