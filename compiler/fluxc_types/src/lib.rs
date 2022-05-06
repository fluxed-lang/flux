mod operator;
mod primitive;

use std::fmt::Debug;

pub use operator::*;
pub use primitive::*;

/// The root-level type expression enumeration
#[derive(Debug, Clone)]
pub enum Type {
    /// A primitive type.
    Primitive(Primitive),
    /// A type operation.
    Operation(Operation),
}

/// The operation enumeration. This enum represents all possible type operations
/// that can be performed on one or more types.
#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    /// An intersection type.
    Intersection(Intersection),
    /// A union type.
    Union(Union),
    /// An array type.
    Array(Box<Type>, Option<usize>),
}

/// Trait implemented by structures that have or represent a Flux type.
pub trait Typed: Debug {
	/// Converts this object into a flux type.
    fn type_of(&self) -> Type;
}

impl<T: Typed> Typed for Box<T> {
    fn type_of(&self) -> Type {
        (**self).type_of()
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self.simplify(), other.simplify()) {
            (Type::Primitive(a), Type::Primitive(b)) => a == b,
            (Type::Operation(a), Type::Operation(b)) => a == b,
            _ => false,
        }
    }
}
