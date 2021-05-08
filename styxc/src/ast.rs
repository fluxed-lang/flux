use std::{collections::HashMap, error::Error};
use crate::lexer::parser;
use crate::errors::print_error;

#[derive(Debug, Clone)]
pub enum Type {
    /// Represents a 64-bit integer type.
    Int64,

    /// Represents a 32-bit integer type.
    Int32,

    /// Represents a 16-bit integer type.
    Int16,

    /// Represents an 8-bit integer type.
    Int8,

    /// Represents an unsigned 64-bit integer type.
    UInt64,

    /// Represents an unsigned 32-bit integer type.
    UInt32,

    /// Represents an unsigned 16-bit integer type.
    UInt16,

    /// Represents an unsigned 8-bit integer type.
    UInt8,

    /// Represents a 128-bit floating point type.
    Float128,

    /// Represents a 64-bit floating point type.
    Float64,

    /// Represents a boolean type.
    Bool
}

pub enum Expr {
    /// Represents a literal type. The second argument is the type of the literal.
    Literal(String, Box<Type>),

    /// Represents an identifier. This could be a variable, class, or function name.
    Identifier(String),

    /// Represents a declaration.
    Declare(String, Box<Type>, Box<Expr>),

    /// Represents a constant declaration.
    DeclareConst(String, Box<Type>, Box<Expr>),

    /// Represents an assignment.
    Assign(String, Box<Expr>),

    /// Represents a primitive type.
    Type(Box<Type>),
    
    /// Represents a binary equality expression.
    Eq(Box<Expr>, Box<Expr>),

    /// Represents a binary inequality expression.
    Ne(Box<Expr>, Box<Expr>),

    /// Represents a binary less-than expression.
    Lt(Box<Expr>, Box<Expr>),

    /// Represents a binary less-than-or-equal expression.
    Le(Box<Expr>, Box<Expr>),

    /// Represents a binary greater-than expression.
    Gt(Box<Expr>, Box<Expr>),
    
    /// Represents a binary greater-than-or-equal expression.
    Ge(Box<Expr>, Box<Expr>),

    /// Represents a binary addition expression.
    Add(Box<Expr>, Box<Expr>),

    /// Represents a binary subtraction expression.
    Sub(Box<Expr>, Box<Expr>),

    /// Represents a binary multiplication expression.
    Mul(Box<Expr>, Box<Expr>),

    /// Represents a binary division expression.
    Div(Box<Expr>, Box<Expr>),

    /// Represents an if statement. The first argument is the condition expression,
    /// the second argument is the statements to execute if this block is true.
    If(Box<Expr>, Vec<Expr>),
    
    /// Represents an if-else statement. The first argument is the condition expression,
    /// the second argument is a vector of statements to execute if the condition is true,
    /// and the third s a vector of statements to execute if the condition expression is false.
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),

    /// Represents a loop block.
    Loop(Box<Option<Expr>>, Vec<Expr>),

    /// Represents a for block.
    /// for (expr; expr; expr) {}
    For(Box<Expr>, Box<Expr>, Box<Expr>, Vec<Expr>),

    /// Represents a function declaration expression.
    Function(String, Vec<String>, String, Vec<Expr>),

    /// Represents a function call.
    Call(String, Vec<Expr>),

    /// Represents a top-level import.
    Import(String, String)
}

/// Represents a variable in the AST.
#[derive(Clone)]
pub struct Var {
    /// The type of this variable.
    field_type: Type,
    /// Whether this variable is a constant.
    constant: bool
}

/// Represents a function declaration in the AST.
#[derive(Clone)]
pub struct Func {
    return_type: Type,
}

/// Represents the current scope for a given block.
#[derive(Clone)]
pub struct Scope {
    /// A hashmap of variables in this scope.
    vars: HashMap<String, Var>,
    /// A hashmap of functions in this scope.
    funcs: HashMap<String, Func>
}

impl Default for Scope {
    fn default() -> Self {
        Self { vars: HashMap::new(), funcs: HashMap::new() }
    }
}

impl Scope {
    pub fn get_expr_type(&self, expr: &Expr) -> Result<Type, Box<dyn Error>> {
        match expr {
            Expr::Literal(_, literal_type) => Ok(*literal_type.clone()),
            // Expr::Function(name, params, v, d) => Ok()
            _ => Err("cannot infer type".into())
        }
    }
}

/// Parse a given input with the PEG parser.
pub fn build_ast(input: String) -> Result<Vec<Expr>, Box<dyn Error>> {
    return match parser::statements(input.as_str()) {
        Ok(statements) => Ok(statements),
        Err(e) => { print_error(input, &e); Err(e.into())}
    }
}

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
            _ => Ok(())
        };
        // validate result
        match parse_result {
            Ok(_) => (),
            Err(e) => return Err(e.into())
        }
    }

    Ok(())
}

/// Validate an AST declaration expression.
fn validate_ast_declare(scope: &mut Scope, name: String, lhs: Box<Type>, value: Box<Expr>) -> Result<(), Box<dyn Error>> {
    // test if variable already exists
    if scope.vars.contains_key(&name) {
        return Err(format!("cannot redeclare variable '{}'", &name).into())
    }
    // if expression is literal, check if they are the same type
    if let Expr::Literal(_, rhs) = *value {
        if !test_types_equal(*lhs.clone(), *rhs.clone()) {
            return Err("types are not equal".into())
        }
    }
    // declare variables in this scope
    scope.vars.insert(name, Var { field_type: *lhs.clone(), constant: false });
    Ok(())
} 

/// Validate an AST assignment expression.
fn validate_ast_assign(scope: &mut Scope, name: String, value: Box<Expr>) -> Result<(), Box<dyn Error>> {
    // test if variable does not exist
    if !scope.vars.contains_key(&name) {
        return Err(format!("cannot assign undeclared variable '{}'", &name).into())
    }
    // if expression is literal, check if they are the same type
    if let Expr::Literal(_, rhs) = *value {
        if !test_var_type_equal(scope.vars.get(&name).unwrap().clone(),*rhs) {
            return Err("types are not equal".into())
        }
    }

    Ok(())
}

/// Test if a variable type is equal to the target type.
pub(crate) fn test_var_type_equal(lhs: Var, rhs: Type) -> bool {
    return test_types_equal(lhs.field_type, rhs)
}

/// Test if the two types are equal.
pub(crate) fn test_types_equal(lhs: Type, rhs: Type) -> bool {
    match (lhs, rhs) {
        (Type::Int64, Type::Int64) => true,
        (Type::Float64, Type::Float64) => true,
        _ => false
    }
}

/// Attempt to fetch the type of the given expression.
pub(crate) fn get_type(expr: &Expr) -> Type {
    match expr {
        Expr::Literal(_, t) => *t.clone(),
        _ => panic!("cannot get type of non-literal expression")
    }
}
