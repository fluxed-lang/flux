use std::error::Error;

use cranelift_module::Linkage;
use log::trace;

use styxc_ast::{AST, Declaration, Expr, Ident, Literal, LiteralKind, Mutability, Node, Stmt, func::ParenArgument, operations::BinOp};
use styxc_types::{Type, equate_types};

/// Represents a callable function.
#[derive(Debug)]
struct Function {
    /// The name of the function.
    name: String,
    /// The arguments of the function.
    args: Vec<ParenArgument>,
    /// The type of the function.
    ty: Type,
    /// The linkage type of this function.
    linkage: Linkage,
}

#[derive(Debug)]
struct Variable {
    /// The name of this variable.
    name: String,
	/// The type of this variable.
	ty: Type,
	/// The mutability of this variable.
	mutability: Mutability,
}

/// Represents a stack.
#[derive(Debug)]
struct Stack<T> {
    /// The contents of the stack.
    contents: Vec<T>,
}

impl <T> Stack<T> {
	/// Creates a new, empty stack.
	fn new() -> Stack<T> {
		Stack {
			contents: Vec::new(),
		}
	}

	fn size(&self) -> usize {
		self.contents.len()
	}

	fn get(&self, index: usize) -> Option<&T> {
		self.contents.get(index)
	}

	fn get_unchecked(&self, index: usize) -> &T {
		self.contents.get(index).unwrap()
	}

	fn push(&mut self, item: T) {
		self.contents.push(item);
	}

	fn pop(&mut self) -> Option<T> {
		self.contents.pop()
	}
}

struct TypeVariable {
    /// The name of this type variable.
    name: String,
    /// The type held by this type variable.
    ty: Type,
}

/// An AST tree walker.
struct TreeWalker {
    ast: AST,
    /// A vector of functions available for calling in the current scope.
    funcs: Vec<Function>,
    /// A stack of variables available to reference in the current scope.
    vars: Stack<Variable>,
    /// A stack of type variables to reference in the current scope.
    ty_vars: Stack<TypeVariable>,
}

impl TreeWalker {
	/// Creates a new tree walker.
	fn new(ast: AST) -> Self {
		Self {
			ast,
			funcs: Vec::default(),
			vars: Stack::new(),
			ty_vars: Stack::new(),
		}
	}

	/// Find a variable by its name.
	pub fn find_var_by_name(&self, name: &str) -> Option<&Variable> {
		for var in self.vars.contents.iter().rev() {
			if var.name == name {
				return Some(var.clone());
			}
		}
		None
	}
	
	/// Find a type variable by its name.
	pub fn find_ty_var_by_name(&self, name: &str) -> Option<&TypeVariable> {
		for ty_var in self.ty_vars.contents.iter().rev() {
			if ty_var.name == name {
				return Some(&ty_var);
			}
		}
		None
	}

    /// Compute the type of an expression.
    fn get_expr_type(&self, expr: &Expr) -> Type {
        trace!("get expression type: {:?}", expr);
        use Expr::*;
        match expr {
            Literal(literal) => self.get_literal_type(&literal.value),
            Ident(ident) => self.find(&ident.value).map_or(Type::Never, |var| var.ty.clone()),
            BinOp(bin_op) => self.get_bin_op_type(&bin_op.value),
            // TODO: Blocks returning things
            Block(_) => Type::Unit,
        }
    }

    /// Fetch the type of a literal.
    fn get_literal_type(&self, literal: &Literal) -> Type {
        trace!("get literal type: {:?}", literal);
        use LiteralKind::*;
        match literal.kind {
            Int(_) => Type::Int,
            Float(_) => Type::Float,
            String(_) => Type::String,
            Char(_) => Type::Char,
            Bool(_) => Type::Bool,
        }
    }
    /// Find a variable identified with the given identifier.
    pub fn find(&self, ident: &Ident) -> Option<&Variable> {
		let size = self.vars.size();

        for i in 0..size {
            let var = &self.vars.get_unchecked(i);
            trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.name);
            if var.name == *ident.name {
                return Some(var);
            }
        }
        None
    }

    /// Test if a variable using the specified identifier exists.
    pub fn exists(&self, ident: &Ident) -> bool {
        self.find(ident).is_some()
    }

    /// Declare a new variable using the specified declaration node and push it onto the stack.
    pub fn declare(&mut self, decl: &mut Declaration) {
        trace!("declare variable: {:?}", decl);
        let ty = self.get_expr_type(&decl.value.value);
        decl.ty = ty;
        let var = Variable {
            ty: self.get_expr_type(&decl.value.value),
            name: decl.ident.value.name.clone(),
            mutability: decl.mutability.clone(),
        };
        self.push(var);
    }

    /// Push a variable onto the stack.
    pub fn push(&mut self, var: Variable) {
        trace!("push to stack: {:?}", var);
        self.vars.push(var)
    }

    /// Pop a variable from the stack.
    pub fn pop(&mut self) -> Option<Variable> {
        self.vars.pop().and_then(|v| {
            trace!("pop from stack: {:?}", v);
            Some(v)
        })
    }

    /// Fetch the type of a binary operation.
    fn get_bin_op_type(&self, bin_op: &BinOp) -> Type {
        trace!("get bin op type: {:?}", bin_op);
        let lhs = self.get_expr_type(&bin_op.lhs.value);
        let rhs = self.get_expr_type(&bin_op.rhs.value);
        if !equate_types(&lhs, &rhs) {
            todo!("bad bin op - error recovery TODO");
        }
        // match comparisons
        use crate::BinOpKind::*;
        match bin_op.kind {
            Eq | Ne | Lt | Gt | Le | Ge => Type::Bool,
            _ => lhs,
        }
    }

    /// Check a statement for symbol resolution errors.
    fn check_stmt(&mut self, stmt: &mut Stmt) -> Result<(), Box<dyn Error>> {
        trace!("check stmt: {:?}", stmt);
        match stmt {
            Stmt::Declaration(decls) => {
                for i in 0..decls.len() {
                    self.declare(&mut decls.get_mut(i).unwrap().value)
                }
                Ok(())
            }
            Stmt::Assignment(assign) => {
                trace!("check assignment: {:?}", assign);
                // fetch types of lhs and rhs
                let lhs = self
                    .find(&assign.value.ident.value)
                    .map_or(Type::Never, |var| var.ty.clone());
                let rhs = self.get_expr_type(&assign.value.value.value);
                // compute types
                if !equate_types(&lhs, &rhs) {
                    Err(format!(
                        "cannot assign type {:?} to variable {} of type {:?}",
                        rhs, assign.value.ident.value.name, lhs
                    )
                    .into())
                } else {
                    Ok(())
                }
            }
            Stmt::If(if_block) => {
                // ensure eexpression is a bool
                if self.get_expr_type(&if_block.value.expr.value) != Type::Bool {
                    return Err("expected bool type for if expression".into());
                }
                self.check_stmts(&mut if_block.value.block.value.stmts)?;
                Ok(())
            }
            Stmt::FuncDecl(func_decl) => {
                self.check_stmts(&mut func_decl.value.body.value.stmts)?;
                Ok(())
            }
            Stmt::FuncCall(func_call) => Ok(()),
            Stmt::ExternFunc(extern_func) => {
                let arg_types: Vec<Type> = extern_func
                    .value
                    .args
                    .iter_mut()
                    .map(|arg| {
                        arg.value.ty = arg.value.ty_ident.value.name.clone().into();
                        arg.value.ty.clone()
                    })
                    .collect();
                // check if there is a return type
                let mut ret_ty = Type::Unit;
                if let Some(ret_ty_ident) = &extern_func.value.ret_ty_ident {
                    ret_ty = ret_ty_ident.value.name.clone().into();
                }

                extern_func.value.ty = Type::Func(arg_types, ret_ty.into());

                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Check a vector of statements for symbol resolution errors. Once all statements are checked,
    /// any declared variables are popped from the stack.
    pub fn check_stmts(&mut self, stmts: &mut Vec<Node<Stmt>>) -> Result<(), Box<dyn Error>> {
        let initial_vars = self.vars.size();
        for i in 0..stmts.len() {
            self.check_stmt(&mut stmts.get_mut(i).unwrap().value)?;
        }
        // pop vars from the stack as they go out of scope.
        for _ in 0..initial_vars {
            self.pop().unwrap();
        }
        Ok(())
    }
}
