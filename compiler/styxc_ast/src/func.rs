use crate::types::Type;

/// Represents a function declaration in the AST.
#[derive(Clone)]
pub struct Func {
    pub return_type: Type,
}
