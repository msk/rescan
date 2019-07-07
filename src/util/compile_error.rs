#[derive(Debug)]
pub struct CompileError {
    pub reason: String,
}

impl CompileError {
    pub(crate) fn new(why: &str) -> Self {
        CompileError {
            reason: why.to_string(),
        }
    }
}
