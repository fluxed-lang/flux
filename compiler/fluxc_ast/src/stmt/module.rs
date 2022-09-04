//! Contains data structures for representing imports and exports.

use crate::{Ident, Node};

/// An imported symbol that forms part of an `import {}` statement.
///
/// This is a data structure that is used to represent the imported symbols in
/// an `import {}` statement. They take the form of `name`, and optionally `name
/// as alias`.
#[derive(Debug, PartialEq, Eq)]
pub struct ImportedSymbol {
    /// The name of the imported symbol.
    pub name: Ident,
    /// The alias this symbol is given in this module.
    pub alias: Option<Ident>,
}

/// An `import` statement.
#[derive(Debug, PartialEq, Eq)]
pub struct Import {
    /// A list of imported symbols.
    pub symbols: Vec<Node<ImportedSymbol>>,
    /// The path to the module being imported.
    pub path: String,
}

/// An `export` statement.
#[derive(Debug, PartialEq, Eq)]
pub struct Export {
    /// A list of exported symbols.
    pub symbols: Vec<Ident>,
}
