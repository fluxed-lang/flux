use std::error::Error;

use log::trace;
use styxc_ast::{operations::Assignment, Block, Declaration, Expr, LiteralKind, Node, Stmt, AST};
use styxc_types::Type;
use styxc_walker::Walker;

/// Check an expression for type errors.
fn check_expr(walker: &mut Walker, expr: &mut Expr) -> Result<Type, Box<dyn Error>> {
    Ok(match expr {
        Expr::Literal(literal) => match literal.value.kind {
            LiteralKind::Bool(_) => Type::Bool,
            LiteralKind::Char(_) => Type::Char,
            LiteralKind::Float(_) => Type::Float,
            LiteralKind::Int(_) => Type::Int,
            LiteralKind::String(_) => Type::String,
        },
        Expr::Ident(ident) => match walker.lookup_variable(&ident.value.name) {
            Some(var) => var.ty.clone(),
            None => {
                return Err(
                    format!("variable with name {} does not exist", ident.value.name).into(),
                )
            }
        },
        Expr::BinOp(bin_op) => {
            let lhs = check_expr(walker, &mut bin_op.value.lhs.value)?;
            let rhs = check_expr(walker, &mut bin_op.value.lhs.value)?;
            lhs.intersect(rhs)
        }
        Expr::Block(block) => {
            // check the block
            check_block(walker, &mut block.value)?;
            Type::Unit
        }
    })
}

/// Check a declaration for type errors.
fn check_declaration(walker: &mut Walker, decl: &mut Declaration) -> Result<(), Box<dyn Error>> {
    // fetch the rhs expression type and put the information into the AST
    decl.ty = check_expr(walker, &mut decl.value.value)?;
    // set the variable type in the walker
    walker
        .lookup_variable_mut(&decl.ident.value.name)
        .unwrap()
        .ty = decl.ty.clone();
    Ok(())
}

/// Check an assignment for type errors.
fn check_assignment(walker: &mut Walker, assign: &mut Assignment) -> Result<(), Box<dyn Error>> {
    let var = walker.lookup_variable(&assign.ident.value.name);
    // check if variable is none
    if let None = var {
        return Err(format!(
            "variable with name {} does not exist",
            assign.ident.value.name
        )
        .into());
    }
    // need to clone here to avoid borrowing twice
    let var_ty = var.unwrap().ty.clone();
    // fetch the type of the expression
    let ty = check_expr(walker, &mut assign.value.value)?;
    // check types are equal
    if var_ty != ty {
        return Err(format!(
            "variable with name {} has type {:?} but expression has type {:?}",
            assign.ident.value.name, var_ty, ty
        )
        .into());
    }
    Ok(())
}

fn check_block(walker: &mut Walker, block: &mut Block) -> Result<(), Box<dyn Error>> {
    // enter the block
    walker.enter_block(&block);
    // iterate over block statements and check for errors
    for stmt in &mut block.stmts {
        check_stmt(walker, stmt)?;
    }
    Ok(())
}

/// Check a statement for type errors.
fn check_stmt(walker: &mut Walker, stmt: &mut Node<Stmt>) -> Result<(), Box<dyn Error>> {
    // tell the walker we are checking the next stmt
    trace!("Checking statement {}", stmt.id);
    let stmt = &mut stmt.value;
    walker.next_stmt(&stmt);
    match stmt {
        Stmt::Declaration(decls) => {
            for decl in decls {
                trace!("Checking declaration {}", decl.id);
                check_declaration(walker, &mut decl.value)?;
            }
        }
        Stmt::Assignment(assignment) => check_assignment(walker, &mut assignment.value)?,
        Stmt::Loop(loop_block) => check_block(walker, &mut loop_block.value.block.value)?,
        Stmt::If(if_block) => check_block(walker, &mut if_block.value.block.value)?,
        Stmt::FuncDecl(_) => todo!(),
        Stmt::ExternFunc(_) => todo!(),
        Stmt::FuncCall(_) => todo!(),
        Stmt::Return(_) => todo!(),
    }
    Ok(())
}

/// Perform the type checking pass.
pub fn perform_ast_type_pass(ast: &mut AST) -> Result<(), Box<dyn Error>> {
    let mut walker = Walker::new();
    // declare top level stmts
    walker.declare_all_in_stmts(&ast.stmts);
    // iterate and descend through AST
    for stmt in &mut ast.stmts {
        check_stmt(&mut walker, stmt)?;
    }
    Ok(())
}
