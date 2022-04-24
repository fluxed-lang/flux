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

/// Trait implemented by types that can be converted into a type expression.
pub trait AsType: Debug {
    fn as_type(&self) -> Type;
}

impl<T: AsType> AsType for &T {
    fn as_type(&self) -> Type {
        (*self).as_type()
    }
}

impl<T: AsType> AsType for Box<T> {
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
