//! Contains data structures for representing imports and exports.

use crate::{Ident, Node};

/// An imported symbol.
#[derive(Debug, PartialEq)]
pub struct ImportedSymbol {
	pub name: Ident,
	pub alias: Option<Ident>
}

/// An `import` statement.
#[derive(Debug, PartialEq)]
pub struct Import {
    /// A list of imported symbols.
    pub symbols: Vec<Node<ImportedSymbol>>,
    /// The path to the module being imported.
    pub path: String,
}

/// An `export` statement.
#[derive(Debug, PartialEq)]
pub struct Export {
    /// A list of exported symbols.
    pub symbols: Vec<Ident>,
}
