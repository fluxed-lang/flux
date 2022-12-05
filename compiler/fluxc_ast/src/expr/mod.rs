//! Contains the expression AST data structures.

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;

pub use block_expr::*;
pub use control::*;
pub use literal::*;
pub use operation::*;

use crate::{FuncCall, Node};

/// The enumeration of possible expression types.
///
/// This enum holds the different kinds of expressions that can occur in Flux
/// source code. Every expression wraps a `Node` which holds the actual data
/// that represents the expression, as well as the span of the source code that
/// the expression represents.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A literal expression.
    Literal(
        /// The inner literal type.
        Node<Literal>,
    ),
    /// An identifier expression.
    Ident(
        /// The inner identifier type.
        Node<Ident>,
    ),
    /// A binary operation expression.
    BinaryExpr(
        /// The inner binary expression type.
        Node<BinaryExpr>,
    ),
    UnaryExpr(Node<UnaryExpr>),
    /// A block of code (e.g. `{ /* ... */ }`).
    Block(
        /// The inner block type.
        Node<Block>,
    ),
    /// A function call expression.
    FuncCall(Node<FuncCall>),
    /// A conditional expression.
    Conditional(Node<Conditional>),
    /// An Unconditional loop expression.
    Loop(Node<Loop>),
    /// A conditional loop expression.
    While(Node<While>),
    /// A match expression.
    Match(Node<Match>),
}

/// The identifier type.
///
/// This type is a simple wrapper around `String` used to represent identifiers
/// within the Flux source code. Identifiers are alphanumeric strings, and may
/// contain underscores. They match the following regex, excluding keywords:
/// ```regex
/// [A-z_][0-9A-z_]*
/// ```
pub type Ident = String;
