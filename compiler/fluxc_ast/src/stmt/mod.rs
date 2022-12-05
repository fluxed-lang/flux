//! Contains the statement AST data structures.

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

pub use declaration::*;
pub use func_decl::*;
pub use module::*;

use crate::{Expr, Node};

/// Enum of possible statement kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// A declaration.
    Declaration(Node<Declaration>),
    /// A type declaration.
    TypeDeclaration(Node<TypeDeclaration>),
    /// A function declaration.
    FuncDecl(Node<FuncDecl>),
    /// A function return statement.
    Return(Node<Expr>),
    /// A loop break statement.
    Break(Node<Expr>),
    /// An import statement.
    Import(Node<Import>),
    /// An export statement.
    Export(Node<Export>),
    /// A generic expression.
    Expr(Node<Expr>),
}
