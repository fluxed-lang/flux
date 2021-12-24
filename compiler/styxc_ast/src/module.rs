//! Contains data structures for representing imports and exports.

use crate::{Ident, Node};

/// An `import` statement.
#[derive(Debug, PartialEq)]
pub struct Import {
    /// A list of imported symbols.
    pub symbols: Vec<(Node<Ident>, Option<Node<Ident>>)>,
    /// The path to the module being imported.
    pub path: String,
}

/// An `export` statement.
#[derive(Debug, PartialEq)]
pub struct Export {
    /// A list of exported symbols.
    pub symbols: Vec<Node<Ident>>,
}
