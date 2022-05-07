use fluxc_types::Type;

use crate::{Block, Expr, Ident, Node};
/// A function call.
#[derive(Debug, PartialEq)]
pub struct FuncCall {
    /// The identifier of the function
    pub ident: Ident,
    /// Arguments being passed to the function.
    pub args: Vec<Node<Expr>>,
}

/// An argument to a function call.
#[derive(Debug, PartialEq, Clone)]
pub struct ParenArgument {
    /// The identifier representing the AST node.
    pub ident: Ident,
    /// The identifier representing the type of this argument.
    pub ty: Node<Type>,
}

/// An enum of function declaration types.
#[derive(Debug, PartialEq)]
pub enum FuncDecl {
    Local {
        /// The identifier representing the function.
        ident: Ident,
        /// The arguments this function requires.
        args: Vec<Node<ParenArgument>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Node<Type>,
    },
    Export {
        /// The identifier representing the function.
        ident: Ident,
        /// The arguments this function requires.
        args: Vec<Node<ParenArgument>>,
        /// The body of the function.
        body: Node<Block>,
        /// The identifier representing the return type of the function.
        ret_ty: Node<Type>,
    },
    External {
        /// The identifier representing the function.
        ident: Ident,
        /// The arguments this function requires.
        args: Vec<Node<ParenArgument>>,
        /// The identifier representing the return type of the function.
        ret_ty: Node<Type>,
    },
}
