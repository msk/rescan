use std::fmt;

/// Error thrown by the compiler.
#[derive(Debug)]
pub struct CompileError {
    kind: ErrorKind,

    /// Reason for the error.
    pub(crate) reason: String,

    /// The index of the expression referred to.
    index: Option<usize>,
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
        Self {
            kind,
            reason: why.into(),
            index: None,
        }
    }

    pub(crate) fn with_index<T: Into<String>>(index: usize, why: T) -> Self {
        Self {
            kind: ErrorKind::Other,
            reason: why.into(),
            index: Some(index),
        }
    }

    pub(crate) fn set_expression_index(&mut self, index: usize) {
        debug_assert!(self.index.is_none());
        self.index = Some(index);
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.index {
            Some(index) => write!(f, "{} in expression {}", self.reason, index),
            None => write!(f, "{}", self.reason),
        }
    }
}
