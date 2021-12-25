//! Contains data structures for representing imports and exports.

use crate::{Node};

/// An external, imported module.
#[derive(Debug, PartialEq, Clone)]
pub struct Module {}

/// An `import` statement.
#[derive(Debug, PartialEq)]
pub struct Import {
    /// A list of imported symbols.
    pub symbols: Vec<(Node<String>, Option<Node<String>>)>,
    /// The path to the module being imported.
    pub path: String,
}

/// An `export` statement.
#[derive(Debug, PartialEq)]
pub struct Export {
    /// A list of exported symbols.
    pub symbols: Vec<Node<String>>,
}
