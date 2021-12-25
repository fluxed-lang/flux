use crate::{types::TypeExpr, Block, Expr, Ident, Node};

/// A function declaration.
#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    /// The identifier representing the function.
    pub ident: Ident,
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The body of the function.
    pub body: Node<Block>,
}

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
    pub type_expr: Node<TypeExpr>,
}

#[derive(Debug, PartialEq)]
pub struct ExternFunc {
    /// The identifier representing the external function.
    pub ident: Ident,
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The identifier representing the return type of the function.
    pub ret_type_expr: Node<TypeExpr>,
}
