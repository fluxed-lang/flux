use crate::ast::Type;
/// Represents a syntax parsing error. These are thrown when the PEG parser
/// encounters an illegal expression pattern while building the AST.
pub struct SyntaxError {
    msg: String,
    line: i64,
}

impl SyntaxError {
    /// Create a new syntax error message.
    pub fn new(msg: String, line: i64) -> Self {
        Self {
            msg,
            line,
        }
    }
}

/// Represents a type error. These are thrown when the AST is parsed and an
/// illegal type expression is encountered (e.g. comparing a bool to an int).
pub struct TypeError {
    msg: String,
    expected: Type,
    actual: Type,
    line: i64
}

impl TypeError {
    /// Create a new type error message.
    pub fn new(msg: String, expected: Type, actual: Type, line: i64) -> Self {
        Self {
            msg, expected, actual, line
        }
    }
}
