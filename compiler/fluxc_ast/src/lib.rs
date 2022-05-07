//! # fluxc_ast
//! Defines AST data structures and types for representing Flux code at compile time.

use fluxc_span::Span;

mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    /// The ID of this node in the AST.
    pub id: usize,
    /// The span of the source code that this node represents.
    pub span: Span,
    /// The inner value held by this AST node.
    pub value: T,
}

impl<T> Node<T> {
    /// Create a new node.
    pub fn new(id: usize, span: Span, value: T) -> Self {
        Self { id, span, value }
    }
}

/// The identifier type.
pub type Ident = Node<String>;

/// The root AST instance.
#[derive(Debug, PartialEq)]
pub struct AST {
    /// The list of top-level statements in the AST.
    pub stmts: Vec<Node<Stmt>>,
}

impl AST {
    /// Create a new AST instance.
    pub fn new() -> AST {
        AST { stmts: vec![] }
    }
}
