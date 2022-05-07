use fluxc_types::Type;

use crate::{Ident, Expr};

/// A declaration of a variable.
#[derive(Debug, PartialEq)]
pub struct Declaration {
    /// The explicit type of this declaration if it exists.
    pub explicit_ty: Option<Type>,
    /// The identifier being declared.
    pub ident: Ident,
    /// The mutability of the declared identifier.
    pub mutability: Mutability,
    /// The declared value.
    pub value: Expr,
}

/// An enum representing variable mutability.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Mutability {
    /// A mutable variable.
    Mutable,
    /// An immutable variable.
    Immutable,
    /// A constant. Unlike an immutable variable, the type of a constant must be defined at compile time, such
    /// that the size of the constant is known.
    Constant,
}
