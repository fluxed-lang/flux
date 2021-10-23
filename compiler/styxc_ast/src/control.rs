use crate::{Block, Expr, Node};

/// An if statement.
#[derive(Debug, PartialEq)]
pub struct If {
    /// The expression this if statement will validate.
    pub expr: Node<Expr>,
    /// The block of code to execute if the expression is true.
    pub block: Node<Block>,
}

/// An else statement.
pub struct Else {
    /// The block this else statement will execute.
    pub block: Node<Block>,
}

/// A match expression.
pub struct Match {
    /// The expression being matched.
    pub expr: Box<Node<Expr>>,
}

#[derive(Debug, PartialEq)]
pub struct Loop {
    /// The block owned by this loop.
    pub block: Node<Block>,
}
