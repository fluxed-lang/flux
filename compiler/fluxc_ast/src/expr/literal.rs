use crate::{Expr};

#[derive(Debug, Clone, PartialEq)]
/// Enum representing the type of a literal.
pub enum Literal {
    /// An integer literal (e.g. `1234`, `0x1234`, `0o1234`, `0b1001`).
    Int(i64),
    /// A floating-point literal (e.g. `1234.5`, `0x1234.5`, `0o1234.5`,
    /// `0b0110.1`).
    Float(f64),
    /// A string literal (e.g. `"hello"`, `"hello world"`).
    String(String),
    /// A character literal (e.g. `'a'`, `'\n'`).
    Char(char),
    /// A boolean literal (e.g. `true`, `false`).
    Bool(bool),
    /// An array literal (e.g. `[1, 2, 3]`).
    Array(Vec<Expr>),
}
