/// Error thrown by the compiler.
#[derive(Debug)]
pub struct CompileError {
    /// Reason for the error.
    pub reason: String,
    kind: ErrorKind,
}

/// A list specifying categories of compile error.
#[derive(Debug)]
pub enum ErrorKind {
    /// Error thrown internally by the Parser interface.
    Parse,
    /// Error thrown internally by the Parser interface with the specific location.
    LocatedParse,
    /// Error thrown by the compiler when an arbitrary resource limit is exceeded.
    ResourceLimit,
    /// Any compile error not part of this list.
    Other,
}

impl CompileError {
    pub(crate) fn new<T: Into<String>>(kind: ErrorKind, why: T) -> Self {
        CompileError {
            reason: why.into(),
            kind,
        }
    }
}
