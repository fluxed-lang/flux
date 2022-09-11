//! Contains the function declaration AST data structures.
//!
//! This module handles:
//! - Local function declarations
//! - External function declarations

use crate::{Block, Expr, Ident, Node};
/// A function call.
#[derive(Debug, PartialEq)]
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
    pub ty: Node<Ident>,
}

/// An enum of function declaration types.
#[derive(Debug, PartialEq)]
pub enum FuncDecl {
    Local {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Option<Node<Ident>>,
    },
    Export {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Option<Node<Ident>>,
    },
    External {
        /// The identifier representing the function.
        ident: Node<Ident>,
        /// The arguments this function requires.
        params: Vec<Node<FuncParam>>,
        /// The identifier representing the return type of the function.
        ret_ty: Option<Node<Ident>>,
    },
}
