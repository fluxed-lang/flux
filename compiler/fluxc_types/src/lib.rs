mod extends;
mod intersect;
mod simplify;
mod union;

use std::fmt::Debug;

pub use extends::*;
pub use intersect::*;
pub use union::*;

/// The root-level type expression enumeration
#[derive(Debug, Clone, PartialEq)]
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
    Intersection(Box<Type>, Box<Type>),
    /// A union type.
    Union(Box<Type>, Box<Type>),
    /// An array type.
    Array(Box<Type>, Option<usize>),
}

/// An enumeration of primitive types. This enum represents all primitive types,
/// including the `never` type.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// The primitive integer type. This represents the infinite union of
    /// all integers.
    Int,
    /// The primitive int literal type.
    IntLiteral(i64),
    /// The primitive string type. This represents the infinite union of all
    /// strings.
    String,
    /// The primitive string literal type.
    StringLiteral(String),
    /// The primitive boolean type.
    Bool,
    /// The primitive boolean literal type, `true`.
    True,
    /// The primitive boolean literal type, `false`.
    False,
    /// The primitive unit type.
    Unit,
    /// The `any` type. This type represents the set of all types.
    Any,
    /// The `never` type. This type represents the empty set.
    Never,
}

impl From<String> for Primitive {
    fn from(s: String) -> Self {
        Primitive::StringLiteral(s)
    }
}

impl From<i64> for Primitive {
    fn from(i: i64) -> Self {
        Primitive::IntLiteral(i)
    }
}

impl From<bool> for Primitive {
    fn from(b: bool) -> Self {
        if b {
            Primitive::True
        } else {
            Primitive::False
        }
    }
}

/// Trait implemented by types that can be converted into a type expression.
pub trait Typed: Debug {
    fn type_of(&self) -> Type;
}

impl<T: Typed> Typed for &T {
    fn type_of(&self) -> Type {
        (*self).type_of()
    }
}

impl<T: Typed> Typed for Box<T> {
    fn type_of(&self) -> Type {
        (**self).type_of()
    }
}

impl Typed for Type {
    fn type_of(&self) -> Type {
        self.clone()
    }
}

impl Typed for Primitive {
    fn type_of(&self) -> Type {
        Type::Primitive(self.clone())
    }
}
