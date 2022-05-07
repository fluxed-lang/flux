use crate::{Ident, Node, Block};

#[derive(Debug, PartialEq)]
pub struct Loop {
    /// The name of this loop.
    pub name: Ident,
    /// The block owned by this loop.
    pub block: Node<Block>,
}

