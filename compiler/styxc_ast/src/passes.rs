use std::error::Error;

use log::{debug, trace};
use styxc_types::{Type, equate_types};

use crate::{AST, BinOp, Declaration, Expr, Ident, Literal, LiteralKind, Mutability, Stmt, StmtKind, Var};

/// A structure holding information about available variables.
#[derive(Default)]
struct SymbolValidator {
    vars: Vec<Var>,
}

impl SymbolValidator {
    /// Find a variable identified with the given identifier.
    pub fn find(&self, ident: &Ident) -> Option<&Var> {
        for i in 0..self.vars.len() {
            let var = &self.vars[self.vars.len() - i - 1];
            trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.ident);
            if var.ident.name == *ident.name {
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
    pub fn declare(&mut self, decl: &Declaration) {
        trace!("declare: {:?}", decl);
        let var = Var {
			ty: Type::Infer,
            ident: decl.ident.clone(),
            mutability: decl.mutability.clone(),
        };
        self.push(var);
    }

    /// Push a variable onto the stack.
    pub fn push(&mut self, var: Var) {
        trace!("push to stack: {:?}", var);
        self.vars.push(var)
    }

    /// Pop a variable from the stack.
    pub fn pop(&mut self) -> Option<Var> {
        self.vars.pop().and_then(|v| {
            trace!("pop from stack: {:?}", v);
            Some(v)
        })
    }

    /// Check an expression for symbol resolution errors.
    fn check_expr(&mut self, expr: &Expr) -> Result<(), Box<dyn Error>> {
        trace!("check expr: {:?}", expr);
        match expr {
            Expr::Literal(_) => Ok(()),
            Expr::Ident(ident) => {
                if self.exists(ident) {
                    Ok(())
                } else {
                    Err(format!("variable {} is used before declaration", ident.name).into())
                }
            }
            Expr::BinOp(bin_op) => {
                self.check_expr(&bin_op.lhs)?;
                self.check_expr(&bin_op.rhs)?;
                Ok(())
            }
            Expr::Block(block) => {
                self.check_stmts(&block.stmts)?;
                Ok(())
            }
        }
    }

    /// Check a statement for symbol resolution errors.
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Box<dyn Error>> {
        trace!("check stmt: {:?}", stmt);
        match &stmt.kind {
            StmtKind::Declaration(decls) => {
                for decl in decls {
                    self.check_expr(&decl.value)?;
                    self.declare(decl);
                }
                Ok(())
            }
            StmtKind::Assignment(assign) => {
                if !self.exists(&assign.ident) {
                    return Err(format!(
                        "variable {} is used before declaration",
                        assign.ident.name
                    )
                    .into());
                }

                if self.find(&assign.ident).unwrap().mutability != Mutability::Mutable {
                    return Err(format!("variable {} is not mutable", assign.ident.name).into());
                }

                self.check_expr(&assign.value)?;
                Ok(())
            }
            StmtKind::Loop(loop_block) => self.check_stmts(&loop_block.block.stmts),
        }
    }

    /// Check a vector of statements for symbol resolution errors. Once all statements are checked,
    /// any declared variables are popped from the stack.
    pub fn check_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Box<dyn Error>> {
        let initial_vars = self.vars.len();
        for stmt in stmts {
            self.check_stmt(stmt)?;
        }
        // pop vars from the stack as they go out of scope.
        for _ in 0..initial_vars {
            self.pop().unwrap();
        }
        Ok(())
    }
}

/// AST pass that ensures symbols are valid and declared properly.
pub fn validate_symbols(ast: &AST) -> Result<(), Box<dyn Error>> {
    debug!("Validating symbol usage...");
    SymbolValidator::default().check_stmts(&ast.stmts)?;
    trace!("Symbols are OK");
    Ok(())
}

/// Stores state for type validation.
#[derive(Default)]
struct TypeValidator {
	vars: Vec<Var>
}

impl TypeValidator {
	   /// Find a variable identified with the given identifier.
    pub fn find(&self, ident: &Ident) -> Option<&Var> {
        for i in 0..self.vars.len() {
            let var = &self.vars[self.vars.len() - i - 1];
            trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.ident);
            if var.ident.name == *ident.name {
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
    pub fn declare(&mut self, decl: &Declaration) {
        let var = Var {
			ty: self.get_expr_type(&decl.value),
            ident: decl.ident.clone(),
            mutability: decl.mutability.clone(),
        };
        self.push(var);
    }

    /// Push a variable onto the stack.
    pub fn push(&mut self, var: Var) {
        self.vars.push(var)
    }

    /// Pop a variable from the stack.
    pub fn pop(&mut self) -> Option<Var> {
        self.vars.pop().and_then(|v| {
            trace!("pop from stack: {:?}", v);
            Some(v)
        })
    }


	/// Compute the type of an expression.
	fn get_expr_type(&self, expr: &Expr) -> Type {
		use Expr::*;
		match expr {
			Literal(literal) => self.get_literal_type(literal),
			Ident(ident) => self.find(ident).map_or(Type::Never, |var| var.ty.clone()),
			BinOp(bin_op) => self.get_bin_op_type(bin_op),
			// TODO: Blocks returning things
			Block(_) => Type::Unit,
		}
	}

	/// Fetch the type of a literal.
	fn get_literal_type(&self, literal: &Literal) -> Type {
		use LiteralKind::*;
		match literal.kind {
   			Int(_) => Type::Int,
   			Float(_) => Type::Float,
   			String(_) => Type::String,
   			Char(_) => Type::Char,
   			Bool(_) => Type::Bool,
		}
	}

	/// Fetch the type of a binary operation.
	fn get_bin_op_type(&self, bin_op: &BinOp) -> Type {
		let lhs = self.get_expr_type(&bin_op.lhs);
		let rhs = self.get_expr_type(&bin_op.rhs);
		if !equate_types(&lhs, &rhs) {
			todo!("bad bin op - error recovery TODO");
		}
		lhs
	}

	/// Check a statement for symbol resolution errors.
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Box<dyn Error>> {
        trace!("check stmt: {:?}", stmt);
        match &stmt.kind {
            StmtKind::Declaration(decls) => {
                for decl in decls {
                    self.declare(decl);
                }
                Ok(())
            }
            StmtKind::Assignment(assign) => {
				trace!("check assignment: {:?}", assign);
				// fetch types of lhs and rhs
				let lhs = self.find(&assign.ident).map_or(Type::Never, |var| var.ty.clone());
				let rhs = self.get_expr_type(&assign.value);
				// compute types
				if !equate_types(&lhs, &rhs) {
					Err(format!("cannot assign type {:?} to variable {} of type {:?}", rhs, assign.ident.name, lhs).into())
				} else {
					Ok(())
				}
            }
            _ => Ok(())
        }
    }

    /// Check a vector of statements for symbol resolution errors. Once all statements are checked,
    /// any declared variables are popped from the stack.
    pub fn check_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Box<dyn Error>> {
        let initial_vars = self.vars.len();
        for stmt in stmts {
            self.check_stmt(stmt)?;
        }
        // pop vars from the stack as they go out of scope.
        for _ in 0..initial_vars {
            self.pop().unwrap();
        }
        Ok(())
    }
}

/// AST pass that ensures types are correct and equivalent.
pub fn validate_types(ast: &AST) -> Result<(), Box<dyn Error>> {
    debug!("Validating types...");
	TypeValidator::default().check_stmts(&ast.stmts)?;
    debug!("Types are OK");
    Ok(())
    // todo!()
}
