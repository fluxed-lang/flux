use fluxc_ast::{FuncDecl, ParenArgument, TypeExpr};

/// HIR datatype representing a Flux function.
///
/// This trait provides utility methods for quickly accessing function
/// information without knowing if it is a class method,
#[derive(Debug)]
pub struct Function {
    /// The name of this function.
    pub name: String,
    /// The kind of this function.
    pub kind: FunctionKind,
    /// The arguments of this function.
    pub args: Vec<Argument>,
    /// The return value of this function.
    pub return_type: TypeExpr,
}

/// Enumeration of function kinds for use in compile-time reflection.
#[derive(Debug, PartialEq)]
pub enum FunctionKind {
    /// A standard function declaration of the form `x -> y`.
    Orphan,
    /// An inline function declaration of the form `x -> y`, that gets inlined
    /// at compile-time.
    InlineOrphan,
    /// An external function declaration.
    External,
    /// A method declaration inside a class.
    Method,
    /// An inline method declaration.
    InlineMethod,
    /// A method definition inside an interface.
    Abstract,
    /// A default method implementation inside an interface.
    Default,
    /// An inline default method implementation inside an interface.
    InlineDefault,
}

/// An argument to a function definition.
#[derive(Debug, PartialEq)]
pub struct Argument {
    pub name: String,
    pub ty: TypeExpr,
}

/// Trait providing the `as_function` method.
pub trait AsFunction {
    /// This method returns `self` as a `Function` type.
    fn as_function(&self) -> Function;
}

impl AsFunction for FuncDecl {
    fn as_function(&self) -> Function {
        match self {
            FuncDecl::Local { ident, params: args, body: _, ret_ty } => Function {
                name: ident.value.clone(),
                args: args.iter().map(|x| (&x.value).into()).collect(),
                kind: FunctionKind::Orphan,
                return_type: ret_ty.clone_inner(),
            },
            FuncDecl::Export { ident: _, args: _, body: _, ret_ty: _ } => {
                todo!("remove FuncDecl::Export")
            }
            FuncDecl::External { ident, params: args, ret_ty } => Function {
                name: ident.value.clone(),
                args: args.iter().map(|x| (&x.value).into()).collect(),
                kind: FunctionKind::External,
                return_type: ret_ty.clone_inner(),
            },
        }
    }
}

impl Into<Argument> for &ParenArgument {
    fn into(self) -> Argument {
        Argument { name: self.ident.clone_inner(), ty: self.ty.clone_inner() }
    }
}
