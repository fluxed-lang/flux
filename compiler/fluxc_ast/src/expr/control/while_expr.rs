use crate::{Block, Expr, Node};

/// A `while` loop.
#[derive(Debug, Clone, PartialEq)]
pub struct While {
    /// The loop condition.
    pub condition: Box<Node<Expr>>,
    /// The block executed every time this loop is executed.
    pub block: Node<Block>,
}
