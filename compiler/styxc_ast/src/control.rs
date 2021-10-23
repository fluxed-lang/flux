use crate::{Block, Expr, Node};

/// An if statement.
#[derive(Debug, PartialEq)]
pub struct If<'a> {
    /// The expression this if statement will validate.
    pub expr: Node<'a, Expr<'a>>,
    /// The block of code to execute if the expression is true.
    pub block: Node<'a, Block<'a>>,
}

/// An else statement.
pub struct Else<'a> {
    /// The block this else statement will execute.
    pub block: Node<'a, Block<'a>>,
}

/// A match expression.
pub struct Match<'a> {
    /// The expression being matched.
    pub expr: Box<Node<'a, Expr<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct Loop<'a> {
    /// The block owned by this loop.
    pub block: Node<'a, Block<'a>>,
}
