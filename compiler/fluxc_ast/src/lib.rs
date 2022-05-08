//! # fluxc_ast
//! Defines AST data structures and types for representing Flux code at compile
//! time.
//!
//! The AST is split into two primary modules:
//! - `fluxc_ast::expr`: Contains the expression AST data structures, or things
//!   that have types.
//! - `fluxc_ast::stmt`: Contains the statement AST data structures.
//!
//! The AST is built from the parser, iterated over by reducers in the
//! `fluxc_ast_passes` crate, before being sent to `fluxc_codegen` and turned
//! into valid LLVM code.

use fluxc_span::Span;
use fluxc_types::{Type, Typed};

mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

/// Wrapper around a generic type `T` that provides an AST ID, and a span.
///
/// This struct is used to wrap AST nodes with their ID, and position in the
/// source code.
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
    /// Create an empty node with no value.
    pub fn empty(id: usize, span: Span) -> Node<()> {
        Node { id, span, value: () }
    }
}

impl Node<()> {
    /// Hydrate this node with the given value.
    pub fn fill<T>(self, value: T) -> Node<T> {
        Node { id: self.id, span: self.span, value }
    }
}

/// Generic implemetation of typed for all nodes
impl<T: Typed> Typed for Node<T> {
    fn type_of(&self) -> Type {
        self.value.type_of()
    }
}

/// The identifier type.
///
/// This type is a simple wrapper around `String` used to represent identifiers
/// within the Flux source code. Identifiers are alphanumeric strings, and may
/// contain underscores. They match the following regex, excluding keywords:
/// ```regex
/// [A-z_][0-9A-z_]*
/// ```
pub type Ident = String;

/// The root AST instance.
///
/// This is the root of the AST, and holds a list of all top-level statements.
/// Instances of this type are created by the parser, and passed to the reducers
/// in the `fluxc_ast_passes` crate, which produce immutable AST instances. The
/// final instance then is sent to the code generator, and turned into valid
/// LLVM code.
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
