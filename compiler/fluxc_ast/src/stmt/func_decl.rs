//! Contains the function declaration AST data structures.
//!
//! This module handles:
//! - Local function declarations
//! - External function declarations

use crate::{Block, Expr, Ident, Node, TypeExpr};

/// An enumeration of function linkage types.
pub enum Linkage {
    Local,
    Export,
    External,
}

/// A function call.
#[derive(Debug, Clone, PartialEq)]
pub struct FuncCall {
    /// The identifier of the function
    pub ident: Node<Ident>,
    /// Arguments being passed to the function.
    pub args: Vec<Node<Expr>>,
}

/// An argument to a function call.
#[derive(Debug, PartialEq, Clone)]
pub struct FuncParam {
    /// The identifier representing the AST node.
    pub ident: Node<Ident>,
    /// The identifier representing the type of this argument.
    pub ty: Node<TypeExpr>,
}

/// An enum of function declaration types.
#[derive(Debug, Clone, PartialEq)]
pub enum FuncDecl {
    Local {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Node<TypeExpr>,
    },
    Export {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Option<Node<TypeExpr>>,
    },
    External {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The identifier representing the return type of the function.
        ret_ty: Option<Node<TypeExpr>>,
    },
}

impl FuncDecl {
    /// Return the linkage of this function.
    pub fn linkage(&self) -> Linkage {
        match self {
            FuncDecl::Local { ident: _, params: _, body: _, ret_ty: _ } => Linkage::Local,
            FuncDecl::Export { ident: _, params: _, body: _, ret_ty: _ } => Linkage::Export,
            FuncDecl::External { ident: _, params: _, ret_ty: _ } => Linkage::External,
        }
    }
}
