use fluxc_ast::Mutability;
use fluxc_types::Type;

#[derive(Debug)]
pub struct Variable {
    /// The name of this variable.
    pub name: String,
    /// The mutability of this variable.
    pub mutability: Mutability,
    /// The type of this variable.
    pub ty: Type,
}
