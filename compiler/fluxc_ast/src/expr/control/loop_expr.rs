use crate::{Block, Ident, Node};

/// A `loop {}` expression.
///
/// This expression is a control flow statement that represents an
/// unconditional loop. This expression takes the return value of the loop body,
/// although the compiler cannot guarantee that the loop body will ever return a
/// value.
#[derive(Debug, PartialEq)]
pub struct Loop {
    /// The name of this loop.
    pub name: Ident,
    /// The block owned by this loop.
    pub block: Node<Block>,
}
