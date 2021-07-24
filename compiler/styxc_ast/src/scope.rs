use std::{collections::HashMap, error::Error};

use crate::{expr::Expr, func::Func, types::Type, var::Var};

/// Represents the current scope for a given block.
#[derive(Clone)]
pub struct Scope<'i> {
    /// A hashmap of variables in this scope.
    pub vars: HashMap<String, &'i Var>,
    /// A hashmap of functions in this scope.
    pub funcs: HashMap<String, &'i Func>,
}

impl Default for Scope<'_> {
    fn default() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
}

impl Scope<'_> {
    pub fn get_expr_type(&self, expr: &Expr) -> Result<Type, Box<dyn Error>> {
        match expr {
            Expr::Literal(_, literal_type) => Ok(*literal_type.clone()),
            // Expr::Function(name, params, v, d) => Ok()
            _ => Err("cannot infer type".into()),
        }
    }
}
