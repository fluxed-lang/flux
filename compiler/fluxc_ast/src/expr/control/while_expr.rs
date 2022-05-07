use crate::{Expr, Node, Block};

/// A `while` loop.
#[derive(Debug, PartialEq)]
pub struct While {
    /// The loop condition.
    pub condition: Box<Expr>,
    /// The block executed every time this loop is executed.
    pub block: Node<Block>,
}
