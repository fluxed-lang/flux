// mod extends;
mod intersect;
mod simplify;
mod union;

// export all types
// pub use extends::*;
pub use intersect::*;
pub use simplify::*;
pub use union::*;

use crate::TypeExpr;

/// The operation enumeration. This enum represents all possible type operations
/// that can be performed on one or more types.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// An intersection type.
    Intersection(Intersection),
    /// A union type.
    Union(Union),
    /// An array type.
    Array(Box<TypeExpr>, Option<usize>),
}

impl Into<TypeExpr> for Operation {
    fn into(self) -> TypeExpr {
        TypeExpr::Operation(self)
    }
}
