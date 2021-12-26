//! # styxc_ast
//! Defines AST data structures and types for representing Styx code at compile time.
use crate::control::{Conditional, Loop, While};
use crate::func::{ExternFunc, FuncCall, FuncDecl};
use crate::module::{Export, Import};
use crate::operations::BinaryExpr;
use crate::types::TypeExpr;
use styxc_span::Span;

pub mod control;
pub mod func;
pub mod module;
pub mod operations;
pub mod types;

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
pub enum Literal {
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
    /// An array literal (e.g. `[1, 2, 3]`).
    Array(Vec<Box<Expr>>),
}

/// The identifier type.
pub type Ident = Node<String>;

/// A `let` declaration.
#[derive(Debug, PartialEq)]
pub struct LetDeclaration {
    /// The identifier used in the declaration.
    pub ident: Ident,
    /// The type of the value being declared.
    pub type_expr: Option<TypeExpr>,
    /// The value being declared.
    pub expr: Expr,
    /// The mutability of the value being declared.
    pub mutable: bool,
}

/// A `mut` declaration.
#[derive(Debug, PartialEq)]
pub struct MutDeclaration {
    /// The identifier used in the declaration.
    pub ident: Ident,
    /// The type of the value being declared.
    pub type_expr: Option<TypeExpr>,
    /// The value being declared.
    pub expr: Expr,
}

/// A `let` or `mut` declaration.
#[derive(Debug, PartialEq)]
pub enum Declaration {
    /// A `let x, mut y` declaration.
    Let(Vec<LetDeclaration>),
    /// A `mut x, y` declaration.
    MutDeclaration(Vec<MutDeclaration>),
}

/// Enum of possible statement kinds.
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// A declaration.
    Declaration(Node<Declaration>),
    /// A function declaration.
    FuncDecl(Node<FuncDecl>),
    /// An external function declaration.
    ExternFunc(Node<ExternFunc>),
    /// A function return statement.
    Return(Expr),
    /// A loop break statement.
    Break(Expr),
    /// An import statement.
    Import(Node<Import>),
    /// An export statement.
    Export(Node<Export>),
    /// A generic expression.
    Expr(Expr),
    /// Defer a function call to the end of this function.
    Defer(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// A literal expression.
    Literal(Node<Literal>),
    /// An identifier expression.
    Ident(Ident),
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
