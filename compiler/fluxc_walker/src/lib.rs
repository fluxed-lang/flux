use fluxc_ast::{
    func::{FuncDecl, ParenArgument},
    Block, Declaration, Expr, Literal, Mutability, Node, Stmt,
};
use fluxc_types::{Intersect, Primitive, Type, Typed};

/// An enum of linkage types.
#[derive(Debug)]
pub enum Linkage {
    /// The function is declared locally, and is not exported.
    Local,
    /// The function is declared in the scope of the module being compiled.
    Module,
    /// The function is declared externally, and has been imported.
    External,
}

/// Represents a callable function.
#[derive(Debug)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The arguments of the function.
    pub args: Vec<ParenArgument>,
    /// The return type of this function.
    pub ret_ty: Type,
}

#[derive(Debug)]
pub struct Variable {
    /// The name of this variable.
    pub name: String,
    /// The mutability of this variable.
    pub mutability: Mutability,
    /// The type of this variable.
    pub ty: Type,
}

/// Represents a type variable declared via the `type x = ` expression.
pub struct TypeVariable {
    /// The name of this type variable.
    name: String,
    /// The type held by this type variable.
    ty: Type,
}

/// Represents a stack.
#[derive(Debug)]
pub struct Stack<T> {
    /// The contents of the stack.
    contents: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new, empty stack.
    pub fn new() -> Stack<T> {
        Stack { contents: Vec::new() }
    }

    /// Return the size of the stack.
    pub fn size(&self) -> usize {
        self.contents.len()
    }

    /// Get an object from the stack.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.contents.get(index)
    }

    /// Get an object from the stack without checking if the stack is empty.
    pub fn get_unchecked(&self, index: usize) -> &T {
        self.contents.get(index).unwrap()
    }

    /// Push an item onto the stack.
    pub fn push(&mut self, item: T) {
        self.contents.push(item);
    }

    /// Pop an item off of the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.contents.pop()
    }

    /// Find an item in the stack using the given predicate.
    pub fn find<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<&T> {
        for item in self.contents.iter().rev() {
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }

    /// Find an item in the stack using the given predicate, returning a mutable
    /// reference to the item.
    pub fn find_mut<F: Fn(&T) -> bool>(&mut self, predicate: F) -> Option<&mut T> {
        for item in self.contents.iter_mut().rev() {
            if predicate(item) {
                return Some(item);
            }
        }
        None
    }
}

/// A utility for walking the AST.
pub struct Walker {
    current_function: Option<Function>,
    variables: Stack<Variable>,
    functions: Stack<Function>,
}

impl Walker {
    /// Create a new walker.
    pub fn new() -> Self {
        Walker { current_function: None, variables: Stack::new(), functions: Stack::new() }
    }

    /// Return the current function. This clones the stored function.
    pub fn current_function(&self) -> Option<&Function> {
        match &self.current_function {
            Some(s) => Some(&s),
            None => None,
        }
    }

    /// Enters the current block, declaring all classes and functions in it.
    pub fn enter_block(&mut self, block: &Block) {
        self.declare_all_in_stmts(&block.stmts);
    }

    /// Declares all functions and classes in the given statements.
    pub fn declare_all_in_stmts(&mut self, stmts: &Vec<Node<Stmt>>) {
        for stmt in stmts {
            match &stmt.value {
                Stmt::FuncDecl(func) => self.declare_function(&func.value),
                _ => (),
            }
        }
    }

    /// Declare a function.
    pub fn declare_function(&mut self, func: &FuncDecl) {
        let (name, args, ret_ty): (String, Vec<ParenArgument>, Type) = match func {
            FuncDecl::Local { ident, args, body, ret_ty } => (
                ident.value.clone(),
                args.iter().map(|arg| arg.value.clone()).collect(),
                ret_ty.value.clone(),
            ),
            FuncDecl::Export { ident, args, body, ret_ty } => (
                ident.value.clone(),
                args.iter().map(|arg| arg.value.clone()).collect(),
                ret_ty.value.clone(),
            ),
            FuncDecl::External { ident, args, ret_ty } => (
                ident.value.clone(),
                args.iter().map(|arg| arg.value.clone()).collect(),
                ret_ty.value.clone(),
            ),
        };

        self.functions.push(Function { args, name, ret_ty });
    }

    /// Declare a variable.
    pub fn declare_variable(&mut self, decl: &Declaration) {
        self.variables.push(Variable {
            name: decl.ident.value.clone(),
            mutability: decl.mutability,
            ty: decl.explicit_ty.clone().unwrap_or(decl.value.type_of()),
        });
    }

    /// Lookup a variable available in the current scope.
    pub fn lookup_variable<S: AsRef<str>>(&self, name: S) -> Option<&Variable> {
        self.variables.find(|v| v.name == name.as_ref())
    }

    /// Lookup a variable available in the current scope, returning a mutable
    /// reference to the variable.
    pub fn lookup_variable_mut<S: AsRef<str>>(&mut self, name: S) -> Option<&mut Variable> {
        self.variables.find_mut(|v| v.name == name.as_ref())
    }

    /// Lookup a funciton available in the current scope.
    pub fn lookup_function(&self, name: &str) -> Option<&Function> {
        self.functions.find(|f| f.name == name.as_ref())
    }

    /// Lookup a funciton available in the current scope.
    pub fn lookup_function_mut(&mut self, name: &str) -> Option<&mut Function> {
        self.functions.find_mut(|f| f.name == name.as_ref())
    }

    /// Get the type of an expression in the current scope.
    pub fn get_expr_type(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::Literal(literal) => match literal.value.kind {
                Literal::Bool(_) => Type::Bool,
                Literal::Int(_) => Type::Int,
                Literal::Float(_) => Type::Float,
                Literal::String(_) => Type::String,
                Literal::Char(_) => Type::Char,
            },
            Expr::Ident(ident) => match self.lookup_variable(&ident.value.inner) {
                Some(var) => var.ty.clone(),
                None => Type::Infer,
            },
            Expr::BinaryExpr(bin_op) => {
                let lhs = self.get_expr_type(&bin_op.value.lhs.value);
                let rhs = self.get_expr_type(&bin_op.value.lhs.value);
                lhs.intersect(&rhs)
            }
            Expr::Block(_) => Type::Primitive(Primitive::Unit),
            Expr::FuncCall(func_call) => func_call.value.return_ty.clone(),
        }
    }

    /// Proceed to the next statement, declaring any variables and functions.
    pub fn next_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Declaration(decls) => {
                for decl in decls {
                    self.declare_variable(&decl.value)
                }
            }
            Stmt::FuncDecl(func) => self.declare_function(&func.value),
            _ => (),
        }
    }
}
