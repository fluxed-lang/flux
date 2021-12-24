//! # styxc_ast
//! Defines AST data structures and types for representing Styx code at compile time.

use control::{Conditional, While};
use module::{Export, Import};
use styxc_types::Type;

use crate::control::Loop;
use crate::func::{ExternFunc, FuncCall, FuncDecl};
use crate::operations::BinaryExpr;
use styxc_span::Span;

pub mod control;
pub mod func;
pub mod module;
pub mod operations;

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

#[derive(Debug, PartialEq)]
/// Enum representing the type of a literal.
pub enum LiteralKind {
    /// An integer literal (e.g. `1234`, `0x1234`, `0o1234`, `0b1001`).
    Int(i64),
    /// A floating-point literal (e.g. `1234.5`, `0x1234.5`, `0o1234.5`, `0b0110.1`).
    Float(f64),
    /// A string literal (e.g. `"hello"`, `"hello world"`).
    String(String),
    /// A character literal (e.g. `'a'`, `'\n'`).
    Char(char),
    /// A boolean literal (e.g. `true`, `false`).
    Bool(bool),
}

impl From<LiteralKind> for Type {
    fn from(kind: LiteralKind) -> Self {
        match kind {
            LiteralKind::Int(_) => Type::Int,
            LiteralKind::Float(_) => Type::Float,
            LiteralKind::String(_) => Type::Array(Type::Char.into()),
            LiteralKind::Char(_) => Type::Char,
            LiteralKind::Bool(_) => Type::Bool,
        }
    }
}

/// A literal value.
#[derive(Debug, PartialEq)]
pub struct Literal {
    /// The type of this literal.
    pub ty: Type,
    /// The kind of literal.
    pub kind: LiteralKind,
}

/// A declaration of a variable.
#[derive(Debug, PartialEq)]
pub struct Declaration {
    /// The type of this declaration.
    pub ty: Type,
    /// The explicit type identifier of this declaration, if it exists.
    pub ty_ident: Option<Node<Ident>>,
    /// The identifier being declared.
    pub ident: Node<Ident>,
    /// The mutability of the declared identifier.
    pub mutability: Mutability,
    /// The declared value.
    pub value: Node<Expr>,
}

/// An enum representing variable mutability.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mutability {
    /// A mutable variable.
    Mutable,
    /// An immutable variable.
    Immutable,
    /// A constant. Unlike an immutable variable, the type of a constant must be defined at compile time, such
    /// that the size of the constant is known.
    Constant,
}

/// An identifier.
#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    /// The name of this node.
    pub name: String,
}

/// Enum of possible statement kinds.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// A declaration.
    Declaration(Vec<Node<Declaration>>),
    /// A function declaration.
    FuncDecl(Node<FuncDecl>),
    /// An external function declaration.
    ExternFunc(Node<ExternFunc>),
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

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A literal expression.
    Literal(Node<Literal>),
    /// An identifier expression.
    Ident(Node<Ident>),
    /// A binary operation expression.
    BinaryExpr(Node<BinaryExpr>),
    /// A block (e.g. `{ /* ... */ }`).
    Block(Node<Block>),
    /// A function call expression.
    FuncCall(Node<FuncCall>),
    /// A conditional expression.
    Conditional(Node<Conditional>),
    /// An Unconditional loop expression.
    Loop(Node<Loop>),
    /// A conditional loop expression.
    While(Node<While>),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    /// The list of statements in the block.
    pub stmts: Vec<Node<Stmt>>,
}

/// An external, imported module.
#[derive(Debug, PartialEq, Clone)]
pub struct Module {}

/// The root AST instance.
#[derive(Debug, PartialEq)]
pub struct AST {
    /// The list of top-level statements in the AST.
    pub stmts: Vec<Node<Stmt>>,
    /// The list of external modules imported into this file.
    pub modules: Vec<Module>,
}

impl AST {
    /// Create a new AST instance.
    pub fn new() -> AST {
        AST {
            stmts: vec![],
            modules: vec![],
        }
    }
}
