// use std::error::Error;

// use log::{debug, trace};
// use styxc_types::{equate_types, Type};

// use crate::{
//     BinOp, Declaration, Expr, ExternFunc, FuncDecl, Ident, Literal, LiteralKind, Mutability,
//     ParenArgument, Stmt, StmtKind, Var, AST,
// };

// /// A structure holding information about available variables.
// #[derive(Default)]
// struct SymbolValidator {
//     vars: Vec<Var>,
// }

// impl SymbolValidator {
//     /// Find a variable identified with the given identifier.
//     pub fn find(&self, ident: &Ident) -> Option<&Var> {
//         for i in 0..self.vars.len() {
//             let var = &self.vars[self.vars.len() - i - 1];
//             trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.ident);
//             if var.ident.name == *ident.name {
//                 return Some(var);
//             }
//         }
//         None
//     }

//     /// Test if a variable using the specified identifier exists.
//     pub fn exists(&self, ident: &Ident) -> bool {
//         self.find(ident).is_some()
//     }

//     /// Declare a new variable using the specified declaration node and push it onto the stack.
//     pub fn declare(&mut self, decl: &Declaration) {
//         trace!("declare: {:?}", decl);
//         let var = Var {
//             ty: Type::Infer,
//             ident: decl.ident.clone(),
//             mutability: decl.mutability.clone(),
//         };
//         self.push(var);
//     }

//     pub fn declare_paren_arg(&mut self, paren_arg: &ParenArgument) {
//         trace!("declare paren arg: {:?}", paren_arg);
//         let var = Var {
//             ty: Type::Infer,
//             ident: paren_arg.ident.clone(),
//             mutability: Mutability::Mutable,
//         };
//         self.push(var)
//     }

//     pub fn declare_func(&mut self, func_decl: FuncDecl) {
//         trace!("declare func: {:?}", func_decl);
//         let var = Var {
//             ty: Type::Infer,
//             ident: func_decl.ident.clone(),
//             mutability: Mutability::Immutable,
//         };
//         self.push(var)
//     }

//     pub fn declare_extern_func(&mut self, extern_func: &ExternFunc) {
//         trace!("declare extern_func: {:?}", extern_func);
//         let var = Var {
//             ty: Type::Infer,
//             ident: extern_func.ident.clone(),
//             mutability: Mutability::Immutable,
//         };
//     }

//     /// Push a variable onto the stack.
//     pub fn push(&mut self, var: Var) {
//         trace!("push to stack: {:?}", var);
//         self.vars.push(var)
//     }

//     /// Pop a variable from the stack.
//     pub fn pop(&mut self) -> Option<Var> {
//         self.vars.pop().and_then(|v| {
//             trace!("pop from stack: {:?}", v);
//             Some(v)
//         })
//     }

//     /// Check an expression for symbol resolution errors.
//     fn check_expr(&mut self, expr: &Expr) -> Result<(), Box<dyn Error>> {
//         trace!("check expr: {:?}", expr);
//         match expr {
//             Expr::Literal(_) => Ok(()),
//             Expr::Ident(ident) => {
//                 if self.exists(ident) {
//                     Ok(())
//                 } else {
//                     Err(format!("variable {} is used before declaration", ident.name).into())
//                 }
//             }
//             Expr::BinOp(bin_op) => {
//                 self.check_expr(&bin_op.lhs)?;
//                 self.check_expr(&bin_op.rhs)?;
//                 Ok(())
//             }
//             Expr::Block(block) => {
//                 self.check_stmts(&block.stmts)?;
//                 Ok(())
//             }
//         }
//     }

//     /// Check a statement for symbol resolution errors.
//     fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Box<dyn Error>> {
//         trace!("check stmt: {:?}", stmt);
//         use StmtKind::*;
//         match &stmt.kind {
//             Declaration(decls) => {
//                 for decl in decls {
//                     self.check_expr(&decl.value)?;
//                     self.declare(decl);
//                 }
//                 Ok(())
//             }
//             Assignment(assign) => {
//                 if !self.exists(&assign.ident) {
//                     return Err(format!(
//                         "variable {} is used before declaration",
//                         assign.ident.name
//                     )
//                     .into());
//                 }

//                 if self.find(&assign.ident).unwrap().mutability != Mutability::Mutable {
//                     return Err(format!("variable {} is not mutable", assign.ident.name).into());
//                 }

//                 self.check_expr(&assign.value)?;
//                 Ok(())
//             }
//             Loop(loop_block) => self.check_stmts(&loop_block.block.stmts),
//             If(if_stmt) => {
//                 self.check_expr(&if_stmt.expr)?;
//                 self.check_stmts(&if_stmt.block.stmts)?;
//                 Ok(())
//             }
//             FuncCall(_) => Ok(()),
//             FuncDecl(decl) => {
//                 for paren_arg in &decl.args {
//                     self.declare_paren_arg(paren_arg);
//                 }
//                 self.check_stmts(&decl.body.stmts)?;
//                 Ok(())
//             }
//             ExternFunc(extern_func) => {
//                 self.declare_extern_func(extern_func);
//                 Ok(())
//             }
//             Return(ret) => self.check_expr(ret),
//         }
//     }

//     /// Check a vector of statements for symbol resolution errors. Once all statements are checked,
//     /// any declared variables are popped from the stack.
//     pub fn check_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), Box<dyn Error>> {
//         let initial_vars = self.vars.len();
//         for stmt in stmts {
//             self.check_stmt(stmt)?;
//         }
//         // if length of vars is equal, we didn't declare any new vars.
//         if initial_vars == self.vars.len() {
//             return Ok(());
//         }
//         // pop vars from the stack as they go out of scope.
//         for _ in 0..initial_vars {
//             self.pop().unwrap();
//         }
//         Ok(())
//     }
// }

// /// AST pass that ensures symbols are valid and declared properly.
// pub fn validate_symbols(ast: &mut AST) -> Result<(), Box<dyn Error>> {
//     debug!("Validating symbol usage...");
//     SymbolValidator::default().check_stmts(&ast.stmts)?;
//     trace!("Symbols are OK");
//     Ok(())
// }

// /// Stores state for type validation.
// #[derive(Default)]
// struct TypeValidator {
//     vars: Vec<Var>,
// }

// impl TypeValidator {
//     /// Find a variable identified with the given identifier.
//     pub fn find(&self, ident: &Ident) -> Option<&Var> {
//         for i in 0..self.vars.len() {
//             let var = &self.vars[self.vars.len() - i - 1];
//             trace!("check ident eq - lhs: {:?} rhs: {:?}", ident, var.ident);
//             if var.ident.name == *ident.name {
//                 return Some(var);
//             }
//         }
//         None
//     }

//     /// Test if a variable using the specified identifier exists.
//     pub fn exists(&self, ident: &Ident) -> bool {
//         self.find(ident).is_some()
//     }

//     /// Declare a new variable using the specified declaration node and push it onto the stack.
//     pub fn declare(&mut self, decl: &mut Declaration) {
//         trace!("declare variable: {:?}", decl);
//         let ty = self.get_expr_type(&decl.value);
//         decl.ty = ty;
//         let var = Var {
//             ty: self.get_expr_type(&decl.value),
//             ident: decl.ident.clone(),
//             mutability: decl.mutability.clone(),
//         };
//         self.push(var);
//     }

//     /// Push a variable onto the stack.
//     pub fn push(&mut self, var: Var) {
//         trace!("push to stack: {:?}", var);
//         self.vars.push(var)
//     }

//     /// Pop a variable from the stack.
//     pub fn pop(&mut self) -> Option<Var> {
//         self.vars.pop().and_then(|v| {
//             trace!("pop from stack: {:?}", v);
//             Some(v)
//         })
//     }

//     /// Compute the type of an expression.
//     fn get_expr_type(&self, expr: &Expr) -> Type {
//         trace!("get expression type: {:?}", expr);
//         use Expr::*;
//         match expr {
//             Literal(literal) => self.get_literal_type(literal),
//             Ident(ident) => self.find(ident).map_or(Type::Never, |var| var.ty.clone()),
//             BinOp(bin_op) => self.get_bin_op_type(bin_op),
//             // TODO: Blocks returning things
//             Block(_) => Type::Unit,
//         }
//     }

//     /// Fetch the type of a literal.
//     fn get_literal_type(&self, literal: &Literal) -> Type {
//         trace!("get literal type: {:?}", literal);
//         use LiteralKind::*;
//         match literal.kind {
//             Int(_) => Type::Int,
//             Float(_) => Type::Float,
//             String(_) => Type::String,
//             Char(_) => Type::Char,
//             Bool(_) => Type::Bool,
//         }
//     }

//     /// Fetch the type of a binary operation.
//     fn get_bin_op_type(&self, bin_op: &BinOp) -> Type {
//         trace!("get bin op type: {:?}", bin_op);
//         let lhs = self.get_expr_type(&bin_op.lhs);
//         let rhs = self.get_expr_type(&bin_op.rhs);
//         if !equate_types(&lhs, &rhs) {
//             todo!("bad bin op - error recovery TODO");
//         }
//         // match comparisons
//         use crate::BinOpKind::*;
//         match bin_op.kind {
//             Eq | Ne | Lt | Gt | Le | Ge => Type::Bool,
//             _ => lhs,
//         }
//     }

//     /// Check a statement for symbol resolution errors.
//     fn check_stmt(&mut self, stmt: &mut Stmt) -> Result<(), Box<dyn Error>> {
//         trace!("check stmt: {:?}", stmt);
//         match &mut stmt.kind {
//             StmtKind::Declaration(decls) => {
//                 for i in 0..decls.len() {
//                     self.declare(decls.get_mut(i).unwrap())
//                 }
//                 Ok(())
//             }
//             StmtKind::Assignment(assign) => {
//                 trace!("check assignment: {:?}", assign);
//                 // fetch types of lhs and rhs
//                 let lhs = self
//                     .find(&assign.ident)
//                     .map_or(Type::Never, |var| var.ty.clone());
//                 let rhs = self.get_expr_type(&assign.value);
//                 // compute types
//                 if !equate_types(&lhs, &rhs) {
//                     Err(format!(
//                         "cannot assign type {:?} to variable {} of type {:?}",
//                         rhs, assign.ident.name, lhs
//                     )
//                     .into())
//                 } else {
//                     Ok(())
//                 }
//             }
//             StmtKind::If(if_block) => {
//                 // ensure eexpression is a bool
//                 if self.get_expr_type(&if_block.expr) != Type::Bool {
//                     return Err("expected bool type for if expression".into());
//                 }
//                 self.check_stmts(&mut if_block.block.stmts)?;
//                 Ok(())
//             }
//             StmtKind::FuncDecl(func_decl) => {
//                 self.check_stmts(&mut func_decl.body.stmts)?;
//                 Ok(())
//             }
//             StmtKind::FuncCall(func_call) => Ok(()),
//             StmtKind::ExternFunc(extern_func) => {
//                 let arg_types: Vec<Type> = extern_func
//                     .args
//                     .iter_mut()
//                     .map(|arg| {
//                         arg.ty = arg.ty_ident.name.clone().into();
//                         arg.ty.clone()
//                     })
//                     .collect();
//                 // check if there is a return type
//                 let mut ret_ty = Type::Unit;
//                 if let Some(ret_ty_ident) = &extern_func.ret_ty_ident {
//                     ret_ty = ret_ty_ident.name.clone().into();
//                 }

//                 extern_func.ty = Type::Func(arg_types, ret_ty.into());

//                 Ok(())
//             }
//             _ => Ok(()),
//         }
//     }

//     /// Check a vector of statements for symbol resolution errors. Once all statements are checked,
//     /// any declared variables are popped from the stack.
//     pub fn check_stmts(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), Box<dyn Error>> {
//         let initial_vars = self.vars.len();
//         for i in 0..stmts.len() {
//             self.check_stmt(stmts.get_mut(i).unwrap())?;
//         }
//         // pop vars from the stack as they go out of scope.
//         for _ in 0..initial_vars {
//             self.pop().unwrap();
//         }
//         Ok(())
//     }
// }

// /// AST pass that ensures types are correct and equivalent.
// pub fn validate_types(ast: &mut AST) -> Result<(), Box<dyn Error>> {
//     debug!("Validating types...");
//     TypeValidator::default().check_stmts(&mut ast.stmts)?;
//     debug!("Types are OK");
//     Ok(())
//     // todo!()
// }
