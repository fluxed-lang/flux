//! Contains the primitive type definitions.

use crate::Type;

/// An enumeration of primitive types. This enum represents all primitive types,
/// including the `never` type.
#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// The primitive integer type. This represents the infinite union of
    /// all integers.
    Int,
    /// The primitive int literal type.
    IntLiteral(i64),
    /// The primitive float type. This represents the infinite union of
    /// all floats.
    Float,
    /// The primitive float literal type.
    FloatLiteral(f64),
    /// The primitive string type. This represents the infinite union of all
    /// strings.
    String,
    /// The primitive string literal type.
    StringLiteral(String),
    /// The primitive char type.
    Char,
    /// The primitive char literal type.
    CharLiteral(char),
    /// A literal tuple type.
    Tuple(Vec<Type>),
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
    /// A literal reference to another type.
    Ref(String),
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

impl Into<Type> for Primitive {
    fn into(self) -> Type {
        Type::Primitive(self)
    }
}

impl Into<Type> for &Primitive {
    fn into(self) -> Type {
        Type::Primitive(self.clone())
    }
}
