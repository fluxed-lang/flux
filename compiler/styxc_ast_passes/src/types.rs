use std::{error::Error, str::FromStr};

use log::trace;
use styxc_ast::{
    func::FuncCall, operations::Assignment, Block, Declaration, Expr, Literal, Node, Stmt, AST,
};
use styxc_types::Type;
use styxc_walker::Walker;

/// Check a function call for errors.
fn check_func_call(
    walker: &mut Walker,
    func_call: &mut Node<FuncCall>,
) -> Result<(), Box<dyn Error>> {
    // lookup function in walker
    let func = walker
        .lookup_function(&func_call.value.ident.value)
        .unwrap();
    // need to clone to avoid borrowing twice
    let func_ty = func.into();
    // ensure arguments match
    if func.args.len() != func_call.value.args.len() {
        return Err(format!(
            "function {} takes {} arguments but {} was given",
            func.name,
            func.args.len(),
            func_call.value.args.len()
        )
        .into());
    }
    // get param tys
    let mut param_tys = vec![];
    for param in func_call.value.args.iter_mut() {
        param_tys.push(check_expr(walker, &mut param.value)?.clone());
    }
    // ensure arg types match
    if let Type::Func(arg_tys, _) = func_ty {
        for i in 0..func_call.value.args.len() {
            let arg_ty = arg_tys.get(i).unwrap();
            let param_ty = param_tys.get(i).unwrap();

            if *arg_ty != *param_ty {
                return Err(format!(
					"function {} takes argument {} of type {:?} but provided value {} has type {:?}",
					func_call.value.ident.value, i, arg_ty, i, param_ty
				)
                .into());
            }
        }
    } else {
        panic!("function type wasn't a function type");
    }
    Ok(())
}

/// Check an expression for type errors.
fn check_expr(walker: &mut Walker, expr: &mut Expr) -> Result<Type, Box<dyn Error>> {
    Ok(match expr {
        Expr::Literal(literal) => match literal.value {
            Literal::Bool(_) => Type::Bool,
            Literal::Char(_) => Type::Char,
            Literal::Float(_) => Type::Float,
            Literal::Int(_) => Type::Int,
            Literal::String(_) => Type::String,
            Literal::Array(_) => todo!(),
        },
        Expr::Ident(ident) => match walker.lookup_variable(&ident.value) {
            Some(var) => var.ty.clone(),
            None => return Err(format!("variable with name {} does not exist", ident.value).into()),
        },
        Expr::BinaryExpr(bin_op) => {
            let lhs = check_expr(walker, &mut bin_op.value.lhs.value)?;
            let rhs = check_expr(walker, &mut bin_op.value.lhs.value)?;
            lhs.intersect(rhs)
        }
        Expr::Block(block) => {
            // check the block
            check_block(walker, &mut block.value)?;
            Type::Unit
        }
        Expr::FuncCall(func_call) => {
            check_func_call(walker, func_call)?;
            // func_call.value.return_ty.clone()
            todo!()
        }
        Expr::Conditional(_) => todo!(),
        Expr::Loop(_) => todo!(),
        Expr::While(_) => todo!(),
    })
}

/// Check a declaration for type errors.
fn check_declaration(walker: &mut Walker, decl: &mut Declaration) -> Result<(), Box<dyn Error>> {
    // fetch the rhs expression type and put the information into the AST
    // decl. = check_expr(walker, &mut decl.value)?;
    // set the variable type in the walker
    walker.lookup_variable_mut(&decl.ident.value).unwrap().ty = decl
        .type_expr
        .as_ref()
        .map_or(Type::Infer, |type_expr| type_expr.clone().into());
    Ok(())
}

/// Check an assignment for type errors.
fn check_assignment(walker: &mut Walker, assign: &mut Assignment) -> Result<(), Box<dyn Error>> {
    let var = walker.lookup_variable(&assign.ident.value);
    // check if variable is none
    if let None = var {
        return Err(format!("variable with name {} does not exist", assign.ident.value).into());
    }
    // need to clone here to avoid borrowing twice
    let var_ty = var.unwrap().ty.clone();
    // fetch the type of the expression
    let ty = check_expr(walker, &mut assign.value)?;
    // check types are equal
    if var_ty != ty {
        return Err(format!(
            "variable with name {} has type {:?} but expression has type {:?}",
            assign.ident.value, var_ty, ty
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
        Stmt::FuncDecl(func_decl) => {
            // // check stmts in function
            // check_block(walker, &mut func_decl.value.body.value)?;
            // // ensure last statement is a return of the correct type
            // let last_stmt = func_decl
            //     .value
            //     .body
            //     .value
            //     .stmts
            //     .last_mut()
            //     .map(|node| &mut node.value);
            // // ensure the last statement exists
            // if let None = last_stmt {
            //     if func_decl.value.ret_type_expr.into() != Type::Unit {
            //         return Err(format!(
            //             "function {} does not return a value",
            //             func_decl.value.ident.value
            //         )
            //         .into());
            //     }
            // }
            // // unwrap the last statement
            // let last_stmt = last_stmt.unwrap();
            // // check last statement is a return
            // if let Stmt::Return(return_stmt) = last_stmt {
            //     // check return type is correct
            //     let return_type = check_expr(walker, &mut return_stmt)?;
            //     if func_decl.value.ret_type_expr.into() != return_type {
            //         return Err(format!(
            //             "function {} returns {:?} but last statement returns {:?}",
            //             func_decl.value.ident.value, func_decl.value, return_type
            //         )
            //         .into());
            //     }
            // } else {
            //     return Err(format!(
            //         "function {} does not return a value",
            //         func_decl.value.ident.value
            //     )
            //     .into());
            // }
            todo!()
        }
        Stmt::ExternFunc(extern_func) => {
            // // compute the argument types
            // let arg_tys: Vec<Type> = extern_func
            //     .value
            //     .args
            //     .iter_mut()
            //     .map(|arg| {
            //         todo!()
            //     })
            //     .collect();
            // // compute the return type of the function.
            // let ret_ty = match &extern_func.value.ret_type_expr {
            //     Some(type_expr) => &type_expr.into(),
            //     None => Type::Unit,
            // };
            // extern_func.value.ty = Type::Func(arg_tys, ret_ty.into());
            // // lookup function in walker
            // let func = walker
            //     .lookup_function_mut(&extern_func.value.ident.value.inner)
            //     .unwrap();
            // func.ty = extern_func.value.ty.clone();
            todo!()
        }
        Stmt::Return(ret) => {
            check_expr(walker, ret)?;
        }
        Stmt::Break(_) => todo!(),
        Stmt::Import(_) => todo!(),
        Stmt::Export(_) => todo!(),
        Stmt::Expr(_) => todo!(),
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
