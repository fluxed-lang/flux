use crate::{Node, Stmt};

/// Represents a block of code, e.g. `{ /* ... */}`.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The list of statements in the block.
    pub stmts: Vec<Node<Stmt>>,
}
