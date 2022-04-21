//! Contains data structures for representing Type expressions.

use fluxc_types::Type;

use crate::{Ident, Node};

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

impl From<TypeLiteral> for Type {
    fn from(lit: TypeLiteral) -> Self {
        match lit {
            TypeLiteral::Unit => Type::Unit,
            TypeLiteral::Bool => Type::Bool,
            TypeLiteral::Int => Type::Int,
            TypeLiteral::Float => Type::Float,
            TypeLiteral::Char => Type::Char,
            TypeLiteral::String => Type::String,
        }
    }
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

impl From<TypeUnaryExpr> for Type {
    fn from(expr: TypeUnaryExpr) -> Self {
        match expr.op {
            TypeUnaryExprOpKind::Array => Type::Array(Box::new(expr.operand.into())),
        }
    }
}

/// The kind of a type binary expression operator.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeBinaryExprOpKind {
    /// The `&` type operator, intersect.
    Intersect,
    /// The `|` type operator, union.
    Union,
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

impl From<TypeBinaryExpr> for Type {
    fn from(expr: TypeBinaryExpr) -> Self {
        match expr.operator {
            TypeBinaryExprOpKind::Intersect => {
                Type::Intersection(vec![expr.lhs.into(), expr.rhs.into()])
            }
            TypeBinaryExprOpKind::Union => Type::Union(vec![expr.lhs.into(), expr.rhs.into()]),
        }
    }
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

impl From<TypeExpr> for Type {
    fn from(expr: TypeExpr) -> Self {
        match expr {
            TypeExpr::Literal(lit) => lit.value.into(),
            TypeExpr::Unary(unary) => unary.value.into(),
            TypeExpr::Binary(binary) => binary.value.into(),
            TypeExpr::Ident(_) => Type::Infer,
        }
    }
}

impl From<Box<TypeExpr>> for Type {
    fn from(expr: Box<TypeExpr>) -> Self {
        expr.into()
    }
}
