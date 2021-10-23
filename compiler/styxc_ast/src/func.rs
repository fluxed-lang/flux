use styxc_types::{FuncType, Type};

use crate::{Block, Expr, Ident, Node};

/// A function declaration.
#[derive(Debug, PartialEq)]
pub struct FuncDecl {
    /// The identifier representing the function.
    pub ident: Node<Ident>,
    /// The type of this function.
    pub ty: Type,
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The return type of the function.
    pub return_ty: Type,
    /// The body of the function.
    pub body: Node<Block>,
}

impl FuncType for FuncDecl {
    fn as_ty(&self) -> Type {
        self.ty.clone()
    }
    fn argument_types(&self) -> Vec<Type> {
        self.args.iter().map(|arg| arg.value.ty.clone()).collect()
    }
    fn ret_ty(&self) -> Type {
        self.return_ty.clone()
    }
}

/// A function call.
#[derive(Debug, PartialEq)]
pub struct FuncCall {
    /// The identifier of the function
    pub ident: Node<Ident>,
    /// Arguments being passed to the function.
    pub args: Vec<Node<Expr>>,
    /// The inferred return type of this function call.
    pub return_ty: Type,
}

/// An argument to a function call.
#[derive(Debug, PartialEq)]
pub struct ParenArgument {
    /// The identifier representing the AST node.
    pub ident: Node<Ident>,
    /// The type of this argument.
    pub ty: Type,
    /// The identifier representing the type of this argument.
    pub ty_ident: Node<Ident>,
}

#[derive(Debug, PartialEq)]
pub struct ExternFunc {
    /// The identifier representing the external function.
    pub ident: Node<Ident>,
    /// The type of this function.
    pub ty: Type,
    /// The arguments this function requires.
    pub args: Vec<Node<ParenArgument>>,
    /// The identifier representing the return type of the function, if there is one.
    pub ret_ty_ident: Option<Node<Ident>>,
}

impl FuncType for ExternFunc {
    fn as_ty(&self) -> Type {
        self.ty.clone()
    }
    fn argument_types(&self) -> Vec<Type> {
        self.args.iter().map(|arg| arg.value.ty.clone()).collect()
    }
    fn ret_ty(&self) -> Type {
        if let Type::Func(_, ret_ty) = &self.ty {
            *ret_ty.clone()
        } else {
            panic!()
        }
    }
}
