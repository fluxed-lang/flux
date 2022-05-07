
pub mod block_expr;
pub mod control;
pub mod literal;
pub mod operation;

pub use block_expr::*;
pub use control::*;
pub use literal::*;
pub use operation::*;

use crate::{Node, FuncCall, Ident};

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A literal expression.
    Literal(Node<Literal>),
    /// An identifier expression.
    Ident(Ident),
    /// A binary operation expression.
    BinaryExpr(Node<BinaryExpr>),
    /// A block (e.g. `{ /* ... */ }`).
    Block(Node<Block>),
    /// A function call expression.
    FuncCall(Node<FuncCall>),
    /// A conditional expression.
    Conditional(Node<Conditional>),
    /// An Unconditional loop expression.
    Loop(Node<Loop>),
    /// A conditional loop expression.
    While(Node<While>),
}
