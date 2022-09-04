use std::{error::Error, str::FromStr};

use crate::{Associativity, Expr, Node};

/// A unary expression.
#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    /// The kind of unary expression.
    pub kind: UnaryOp,
    /// The operand of the unary expression.
    pub expr: Box<Node<Expr>>,
}

/// Enum representing unary operator types.
///
/// Unary operators are operators that act on a single argument, such as `x++`,
/// or `!x`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryOp {
    /// The suffix increment operator, `++`.
    Increment,
    /// The suffix decrement operator, `--`.
    Decrement,
    /// The prefix increment operator, `++`.
    /// The index operator, `[n]`
    Index(u64),
    /// The address-of operator, `&`.
    Reference,
    /// The bitwise not operator, `~`.
    BitwiseNot,
    /// The logical not operator, `!`.
    LogicalNot,
    /// The de-reference operator, `*`.
    Dereference,
    /// The negation operator, `-`.
    Negation,
}

impl FromStr for UnaryOp {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnaryOp::*;

        // match index operator
        if s.starts_with('[') && s.ends_with(']') {
            let mut chars = s.chars();
            chars.next();
            chars.next_back();
            let inner: String = chars.collect();
            let index: u64 = inner.parse::<u64>().unwrap_or(0);
            return Ok(Index(index));
        }

        match s {
            "++" => Ok(Increment),
            "--" => Ok(Decrement),
            "&" => Ok(Reference),
            "~" => Ok(BitwiseNot),
            "!" => Ok(LogicalNot),
            "*" => Ok(Dereference),
            _ => Err("invalid unary operator".into()),
        }
    }
}

impl UnaryOp {
    /// Fetch the precedence of this unary operator.
    pub const fn precedence(&self) -> usize {
        use UnaryOp::*;
        match self {
            Increment | Decrement | Index(_) => 1,
            _ => 2,
        }
    }

    /// Fetch the associativity of this unary operator.

    pub const fn associativity(&self) -> Associativity {
        use UnaryOp::*;
        match self {
            Increment | Decrement | Index(_) => Associativity::Ltr,
            _ => Associativity::Rtl,
        }
    }
}
