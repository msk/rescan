#[derive(Clone)]
pub(crate) struct Grey {
    pub(crate) limit_pattern_count: usize,
    pub(crate) limit_pattern_length: usize,
}

impl Default for Grey {
    fn default() -> Self {
        Self {
            limit_pattern_count: 8_000_000,
            limit_pattern_length: 16_000,
        }
    }
}
