use styxc_span::Span;
use thiserror::Error;

/// An enum of all possible errors thrown by the compiler.
#[derive(Debug, Error, Clone)]
pub enum ErrorKind {
    /// E0001 - Not Implemented.
    #[error("E0001 - the feature `{0}` has not been implemented")]
    E0001(String),
    /// E0100 - Expected token.
    #[error("E0100 - expected `{0}`")]
    E0100(String),
    /// E0101 - Expected token, but found token.
    #[error("E0101 - expected `{0}`, but found `{1}`")]
    E0101(String, String),
}

/// A fatal error thrown by the compiler.
pub struct FatalError {
    /// The error kind.
    pub kind: ErrorKind,
    /// The span of the source file that caused this error.
    pub span: Span,
}

/// An enum of possible error types.
pub enum CompilerError {
    Fatal(FatalError),
}

impl FatalError {
    /// Creates a new fatal error.
    pub fn new(span: Span, kind: ErrorKind) -> FatalError {
        FatalError { span, kind }
    }
}
