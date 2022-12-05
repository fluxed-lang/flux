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

use std::ops::Range;

use chumsky::Span;
use fluxc_types::{Type, Typed};

mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

/// Wrapper around a generic type `T` that provides an AST ID, and a span.
///
/// This struct is used to wrap AST nodes with their ID, and position in the
/// source code.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node<T> {
    /// The inner value held by this AST node.
    pub value: T,
    /// The span of the source code that this node represents.
    pub span: Range<usize>,
}

impl<T> Node<T> {
    /// Create a new node.
    pub fn new(value: T, span: Range<usize>) -> Self {
        Self { value, span }
    }
    /// Create an empty node with no value.
    pub fn empty(span: Range<usize>) -> Node<()> {
        Node { value: (), span }
    }
}

impl<T: Clone> Node<T> {
    /// This method clones the inner value of the node and returns it.
    pub fn clone_inner(&self) -> T {
        self.value.clone()
    }
}

// generic implemetation of typed for all nodes
impl Node<()> {
    /// Hydrate this node with the given value.
    pub fn fill<T>(self, value: T) -> Node<T> {
        Node { span: self.span, value }
    }
}

impl<T: Clone> Span for Node<T> {
    type Context = T;
    type Offset = usize;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Node { value: context, span: range }
    }

    fn context(&self) -> Self::Context {
        self.value.clone()
    }

    fn start(&self) -> Self::Offset {
        self.span.start
    }

    fn end(&self) -> Self::Offset {
        self.span.end
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
