use std::error::Error;

use crate::AST;

/// AST pass that ensures symbols are valid and declared properly.
pub fn validate_symbols(ast: &AST) -> Result<(), Box<dyn Error>> {
    todo!()
}

/// AST pass that ensures types are correct and equivalent.
pub fn validate_types(ast: &AST) -> Result<(), Box<dyn Error>> {
    todo!()
}
