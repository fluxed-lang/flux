//! Contains data structures for representing Type expressions.

use crate::{Ident, Node};

/// The kind of a type binary expression operator.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeBinaryExprOpKind {
    /// The `&` type operator, intersect.
    Intersect,
    /// The `|` type operator, union.
    Union,
}

/// An enum of type literals.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeLiteral {
    /// The unit type, `()`.
    Unit,
    /// The boolean type, `bool`.
    Bool,
    /// The integer type, `int`.
    Int,
    /// The floating-point type, `float`.
    Float,
    /// The character type, `char`.
    Char,
    /// The string type, `string`.
    String,
}

/// The kind of a type unary expression.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeUnaryExprOpKind {
    /// The array operator, `[]`.
    Array,
}

/// A type unary expression.
#[derive(Debug, PartialEq, Clone)]
pub struct TypeUnaryExpr {
    /// The operator of this unary expression.
    pub op: TypeUnaryExprOpKind,
    /// The operand of this unary expression.
    pub operand: Box<TypeExpr>,
}

/// A type binary expression.
#[derive(Debug, PartialEq, Clone)]
pub struct TypeBinaryExpr {
    /// The left-hand side of the binary expression.
    pub lhs: Box<TypeExpr>,
    /// The operator of the binary expression.
    pub operator: TypeBinaryExprOpKind,
    /// The right-hand side of the binary expression.
    pub rhs: Box<TypeExpr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeExpr {
    /// A type literal.
    Literal(Node<TypeLiteral>),
    /// A type unary expression.
    Unary(Node<TypeUnaryExpr>),
    /// A type binary expression.
    Binary(Node<TypeBinaryExpr>),
    /// An identifier.
    Ident(Node<Ident>),
}
