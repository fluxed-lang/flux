pub(crate) mod binary_expr;
pub(crate) mod unary_expr;

pub use binary_expr::*;
pub use unary_expr::*;

/// Enum representing operator associativity.
///
/// Some operators are evaluated from left-to-right, while others are evaluated
/// from right-to-left. This property is known as an operator's associativity.
/// In order for the compiler to correctly generate machine code that performs
/// as expected, the associativity of each operator must be defined
/// in the language specification.
///
/// This enum contains two values:
/// - `Associativity::Left`: The left-to-right associativity.
/// - `Associativity::Right`: The right-to-left associativity.
///
/// Each operator is then matched to either one of these options, and compiled
/// as such.
#[derive(Debug, PartialEq)]
pub enum Associativity {
    /// Left-to-right associativity.
    Ltr,
    /// Right-to-left associativity.
    Rtl,
}
