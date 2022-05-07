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
    Infer(String),
    /// A circular type reference.
    Circular(Box<Type>),
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
