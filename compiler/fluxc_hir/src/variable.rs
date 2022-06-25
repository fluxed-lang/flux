use fluxc_ast::{Mutability, TypeExpr};

#[derive(Debug)]
pub struct Variable {
    /// The name of this variable.
    pub name: String,
    /// The mutability of this variable.
    pub mutability: Mutability,
    /// The type of this variable.
    pub ty: TypeExpr,
}
