use std::error::Error;

use log::{debug, trace};

use crate::{Declaration, Expr, Ident, Stmt, StmtKind, Var, AST};

/// A structure holding information about available variables.
#[derive(Default)]
struct SymbolValidator {
    vars: Vec<Var>,
}

impl SymbolValidator {
    /// Test if a variable using the specified identifier exists.
    pub fn exists(&self, ident: &Ident) -> bool {
        for var in &self.vars {
            trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.ident);
            if var.ident.name == *ident.name {
                return true;
            }
        }
        false
    }

    /// Declare a new variable using the specified declaration node and push it onto the stack.
    pub fn declare(&mut self, decl: &Declaration) {
        trace!("declare: {:?}", decl);
        let var = Var {
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
    SymbolValidator::default().check_stmts(&ast.stmts)
}

/// AST pass that ensures types are correct and equivalent.
pub fn validate_types(ast: &AST) -> Result<(), Box<dyn Error>> {
    debug!("Validating types...");
    Ok(())
    // todo!()
}
