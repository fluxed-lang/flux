use cranelift_module::Linkage;

use styxc_ast::{func::ParenArgument, AST};
use styxc_types::Type;

/// Represents a callable function.
struct Function<'a> {
    /// The name of the function.
    name: String,
    /// The arguments of the function.
    args: Vec<ParenArgument<'a>>,
    /// The type of the function.
    ty: Type,
    /// The linkage type of this function.
    linkage: Linkage,
}

struct Variable {
    /// The name of this variable.
    name: String,
}

/// Represents a stack.
struct Stack<T> {
    /// The contents of the stack.
    contents: Vec<T>,
}

struct TypeVariable {
    /// The name of this type variable.
    name: String,
    /// The type held by this type variable.
    ty: Type,
}

/// An AST tree walker.
struct TreeWalker<'a> {
    ast: AST<'a>,
    /// A vector of functions available for calling in the current scope.
    funcs: Vec<Function<'a>>,
    /// A stack of variables available to reference in the current scope.
    vars: Stack<Variable>,
    /// A stack of type variables to reference in the current scope.
    ty_vars: Stack<TypeVariable>,
}
