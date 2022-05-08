use std::{error::Error, str::FromStr};

use crate::{Associativity, Expr, Node};

/// A binary expression.
///
/// This struct represents a binary operation that takes two operands and
/// returns a result.
#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    /// The left hand side of the binary expression.
    pub lhs: Box<Node<Expr>>,
    /// The right hand side of the binary expression.
    pub rhs: Box<Node<Expr>>,
    /// The kind of binary expression.
    pub kind: BinaryOp,
}

/// An enumeration of possible binary operations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BinaryOp {
    /// The addition operator, `+`.
    Plus,
    /// The subtraction operator, `-`.
    Minus,
    /// The multiplication operator, `*`.
    Mul,
    /// The division operator, `/`.
    Div,
    /// The modulo operator, `%`.
    Mod,
    /// The bitwise AND operator, `&`.
    BitwiseAnd,
    /// The bitwise OR operator, `|`.
    BitwiseOr,
    /// The bitwise XOR operator, `^`.
    BitwiseXor,
    /// The logical AND operator, `&&`.
    LogicalAnd,
    /// The logical OR operator, `||`.
    LogicalOr,
    /// The bitwise left shift operator, `<<`.
    Shl,
    /// The bitwise right shift operator, `>>`.
    Shr,
    /// The equality operator, `==`.
    Eq,
    /// The inequality operator, `!=`.
    Ne,
    /// The less-than operator, `<`.
    Lt,
    /// The greater-than operator, `>`.
    Gt,
    /// The less-than-or-equal operator, `<=`.
    Le,
    /// The greater-than-or-equal operator, `>=`.
    Ge,
    /// The assignment operator, `=`.
    Assign,
    /// The assignment operator, `+=`.
    PlusEq,
    /// The assignment operator, `-=`.
    MinusEq,
    /// The assignment operator, `*=`.
    MulEq,
    /// The assignment operator, `/=`.
    DivEq,
    /// The assignment operator, `%=`.
    ModEq,
    /// The assignment operator, `&=`.
    BitwiseAndEq,
    /// The assignment operator, `|=`.
    BitwiseOrEq,
    /// The assignment operator, `^=`.
    BitwiseXorEq,
	/// The assignment operator `&&=`.
	LogicalAndEq,
	/// The assignment operator `||=`.
	LogicalOrEq,
    /// The assignment operator, `<<=`.
    ShlEq,
    /// The assignment operator, `>>=`.
    ShrEq,
}

impl FromStr for BinaryOp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<BinaryOp, Self::Err> {
        use BinaryOp::*;
        match s {
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Mod),
            "&" => Ok(BitwiseAnd),
            "|" => Ok(BitwiseOr),
            "^" => Ok(BitwiseXor),
            "<<" => Ok(Shl),
            ">>" => Ok(Shr),
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            "<" => Ok(Lt),
            ">" => Ok(Gt),
            "<=" => Ok(Le),
            ">=" => Ok(Ge),
            "=" => Ok(Assign),
            "+=" => Ok(PlusEq),
            "-=" => Ok(MinusEq),
            "*=" => Ok(MulEq),
            "/=" => Ok(DivEq),
            "%=" => Ok(ModEq),
            "&=" => Ok(BitwiseAndEq),
            "|=" => Ok(BitwiseOrEq),
            "^=" => Ok(BitwiseXorEq),
            "<<=" => Ok(ShlEq),
            ">>=" => Ok(ShrEq),
            _ => Err("invalid binary operator".into()),
        }
    }
}

impl BinaryOp {
    /// Get the precedence of this binary operator.
    pub const fn precedence(&self) -> usize {
        match self {
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 5,
            BinaryOp::Plus | BinaryOp::Minus => 6,
            BinaryOp::Shl | BinaryOp::Shr => 7,
            BinaryOp::BitwiseAnd => 8,
            BinaryOp::BitwiseXor => 9,
            BinaryOp::BitwiseOr => 10,
            BinaryOp::Lt | BinaryOp::Gt | BinaryOp::Le | BinaryOp::Ge => 11,
            BinaryOp::Eq | BinaryOp::Ne => 12,
            BinaryOp::LogicalAnd => 13,
            BinaryOp::LogicalOr => 14,
            _ => 15,
        }
    }

    /// Get the associativity of this binary operator.
    pub const fn associativity(&self) -> Associativity {
        match self {
            _ => Associativity::Ltr,
        }
    }
}
