use std::error::Error;

use styxc_types::Type;

use crate::{expr::Expr, scope::Scope, var::Var};

/// Recursively descend through the AST and ensure all types are correct.
pub fn validate_ast(scope: &mut Scope, expressions: Vec<Expr>) -> Result<(), Box<dyn Error>> {
    // keep a record of variables in this scope.
    for expr in expressions {
        // use Expr for short-hand access to enum keys.
        use Expr::*;
        // match the expression type and validate it.
        let parse_result = match expr {
            Declare(name, lhs, value) => validate_ast_declare(scope, name, lhs, value),
            Assign(name, value) => validate_ast_assign(scope, name, value),
            _ => Ok(()),
        };
        // validate result
        match parse_result {
            Ok(_) => (),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

/// Validate an AST declaration expression.
fn validate_ast_declare(
    scope: &mut Scope,
    name: String,
    lhs: Box<Type>,
    value: Box<Expr>,
) -> Result<(), Box<dyn Error>> {
    // test if variable already exists
    if scope.vars.contains_key(&name) {
        return Err(format!("cannot redeclare variable '{}'", &name).into());
    }
    // if expression is literal, check if they are the same type
    if let Expr::Literal(_, rhs) = *value {
        if !test_types_equal(*lhs.clone(), *rhs.clone()) {
            return Err("types are not equal".into());
        }
    }
    // declare variables in this scope
    scope.vars.insert(
        name,
        Var {
            field_type: *lhs.clone(),
            constant: false,
        },
    );
    Ok(())
}

/// Validate an AST assignment expression.
fn validate_ast_assign(
    scope: &mut Scope,
    name: String,
    value: Box<Expr>,
) -> Result<(), Box<dyn Error>> {
    // test if variable does not exist
    if !scope.vars.contains_key(&name) {
        return Err(format!("cannot assign undeclared variable '{}'", &name).into());
    }
    // if expression is literal, check if they are the same type
    if let Expr::Literal(_, rhs) = *value {
        if !test_var_type_equal(scope.vars.get(&name).unwrap().clone(), *rhs) {
            return Err("types are not equal".into());
        }
    }

    Ok(())
}

/// Test if a variable type is equal to the target type.
pub fn test_var_type_equal(lhs: Var, rhs: Type) -> bool {
    return test_types_equal(lhs.field_type, rhs);
}

/// Test if the two types are equal.
pub fn test_types_equal(lhs: Type, rhs: Type) -> bool {
    match (lhs, rhs) {
        (Type::Int64, Type::Int64) => true,
        (Type::Float64, Type::Float64) => true,
        _ => false,
    }
}

/// Attempt to fetch the type of the given expression.
pub fn get_type(expr: &Expr) -> Type {
    match expr {
        Expr::Literal(_, t) => *t.clone(),
        _ => panic!("cannot get type of non-literal expression"),
    }
}
