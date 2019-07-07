mod util;

pub use util::compile_error::CompileError;

/// Compiles a regular expression.
pub fn compile(_expression: &str) -> Result<(), CompileError> {
    Err(CompileError::new("not implemented"))
}

/// Compiles multiple regular expressions.
pub fn compile_multi(_expressions: &[&str], _ids: &[u32]) -> Result<(), CompileError> {
    Err(CompileError::new("not implemented"))
}
