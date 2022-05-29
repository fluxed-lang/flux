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
    /// A type to be inferred from the type tree.
    Infer,
    /// A circular type reference.
    Circular(Box<Type>),
}

/// Trait implemented by structures that have or represent a Flux type.
pub trait Typed: Debug + Sized {
    /// This method returns this object as a flux type.
    fn as_type(&self) -> Type;
    /// This method consumes this object and returns its type representation.
    fn into_type(self) -> Type {
        self.as_type()
    }
}

impl<T: Typed> Typed for Box<T> {
    fn as_type(&self) -> Type {
        (**self).as_type()
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
