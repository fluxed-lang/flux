use crate::{Expr, Ident, Node, TypeExpr};

/// A declaration of a variable.
///
/// This structure handles the declaration of variables in Flux source code. A
/// declaration looks something like:
/// ```flx
/// let x = 1
/// ```
/// Declarations can be either immutable or mutable, depending on if the `mut`
/// keyword is specified.
/// ```flx
/// let x = 1
/// x = 2 // error!
/// let mut x = 1
/// x = 2 // ok
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    /// The explicit type of this declaration if it exists.
    pub explicit_ty: Option<Node<TypeExpr>>,
    /// The identifier being declared.
    pub ident: Node<Ident>,
    /// The mutability of the declared identifier.
    pub mutability: Mutability,
    /// The declared value.
    pub value: Node<Expr>,
}

/// An enum representing variable mutability.
///
/// This enum is used in the [`Declaration`](#declaration) struct to determine
/// its mutability.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mutability {
    /// A mutable variable.
    Mutable,
    /// An immutable variable.
    Immutable,
    /// A constant. Unlike an immutable variable, the type of a constant must be
    /// defined at compile time, such that the size of the constant is
    /// known.
    Constant,
}
