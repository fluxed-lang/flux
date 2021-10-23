use std::{error::Error, str::FromStr};

use crate::{Expr, Ident, Node};

/// Enum representing operator associativity.
///
/// Some operators are evaluated from left-to-right, while others are evaluated from right-to-left.
/// This property is known as an operator's associativity. In order for the compiler to correctly
/// generate machine code that performs as expected, the associativity of each operator must be defined
/// in the language specification.
///
/// This enum contains two values:
/// - `Associativity::Left`: The left-to-right associativity.
/// - `Associativity::Right`: The right-to-left associativity.
///
/// Each operator is then matched to either one of these options, and compiled as such.
#[derive(Debug, PartialEq)]
pub enum Associativity {
    /// Left-to-right associativity.
    Ltr,
    /// Right-to-left associativity.
    Rtl,
}

/// Enum representing unary operator types.
///
/// Unary operators are operators that act on a single argument, such as `x++`, or `!x`.
#[derive(Debug, PartialEq)]
pub enum UnOpKind {
    /// The suffix increment operator, `++`.
    Incr,
    /// The suffix decrement operator, `--`.
    Decr,
    /// The prefix increment operator, `++`.
    /// The index operator, `[n]`
    Index(usize),
    /// The address-of operator, `&`.
    Addr,
    /// The bitwise not operator, `~`.
    Not,
    /// The logical not operator, `!`.
    LogNot,
    /// The de-reference operator, `*`.
    Deref,
    /// The negation operator.
    Neg,
}

impl FromStr for UnOpKind {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UnOpKind::*;

        // match index operator
        if s.starts_with("[") && s.ends_with("]") {
            let mut chars = s.chars();
            chars.next();
            chars.next_back();
            let inner: String = chars.collect();
            let index: usize = inner.parse::<usize>().unwrap_or(0);
            return Ok(Index(index));
        }

        match s {
            "++" => Ok(Incr),
            "--" => Ok(Decr),
            "&" => Ok(Addr),
            "~" => Ok(Not),
            "!" => Ok(LogNot),
            "*" => Ok(Deref),
            _ => Err("invalid unary operator".into()),
        }
    }
}

impl UnOpKind {
    /// Fetch the precedence of this unary operator.
    pub const fn precedence(&self) -> usize {
        use UnOpKind::*;
        match self {
            Incr | Decr | Index(_) => 1,
            _ => 2,
        }
    }

    /// Fetch the associativity of this unary operator.

    pub const fn associativity(&self) -> Associativity {
        use UnOpKind::*;
        match self {
            Incr | Decr | Index(_) => Associativity::Ltr,
            _ => Associativity::Rtl,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BinOpKind {
    /// The addition operator, `+`.
    Add,
    /// The subtraction operator, `-`.
    Sub,
    /// The multiplication operator, `*`.
    Mul,
    /// The division operator, `/`.
    Div,
    /// The modulo operator, `%`.
    Mod,
    /// The bitwise AND operator, `&`.
    And,
    /// The bitwise OR operator, `|`.
    Or,
    /// The bitwise XOR operator, `^`.
    Xor,
    /// The logical AND operator, `&&`.
    LogAnd,
    /// The logical OR operator, `||`.
    LogOr,
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
}

impl FromStr for BinOpKind {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<BinOpKind, Self::Err> {
        use BinOpKind::*;
        match s {
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Mod),
            "&" => Ok(And),
            "|" => Ok(Or),
            "^" => Ok(Xor),
            "<<" => Ok(Shl),
            ">>" => Ok(Shr),
            "==" => Ok(Eq),
            "!=" => Ok(Ne),
            "<" => Ok(Lt),
            ">" => Ok(Gt),
            "<=" => Ok(Le),
            ">=" => Ok(Ge),
            _ => Err("invalid binary operator".into()),
        }
    }
}

impl BinOpKind {
    /// Fetch the precedence of this binary operator.
    pub const fn precedence(&self) -> usize {
        match self {
            BinOpKind::Mul | BinOpKind::Div | BinOpKind::Mod => 3,
            BinOpKind::Add | BinOpKind::Sub => 4,
            BinOpKind::Shl | BinOpKind::Shr => 5,
            BinOpKind::Lt | BinOpKind::Gt | BinOpKind::Le | BinOpKind::Ge => 6,
            BinOpKind::Eq | BinOpKind::Ne => 7,
            BinOpKind::And => 8,
            BinOpKind::Xor => 9,
            BinOpKind::Or => 10,
            BinOpKind::LogAnd => 11,
            BinOpKind::LogOr => 12,
        }
    }

    /// Fetch the associativity of this binary operator.
    pub const fn associativity(&self) -> Associativity {
        match self {
            _ => Associativity::Ltr,
        }
    }
}

/// A binary expression.
#[derive(Debug, PartialEq)]
pub struct BinOp<'a> {
    /// The left hand side of the binary expression.
    pub lhs: Box<Node<'a, Expr<'a>>>,
    /// The right hand side of the binary expression.
    pub rhs: Box<Node<'a, Expr<'a>>>,
    /// The kind of binary expression.
    pub kind: BinOpKind,
}

#[derive(Debug, PartialEq)]
pub enum AssignmentKind {
    /// The assignment operator, `=`.
    Assign,
    /// The bitwise left-shift assignment operator, `<<=`.
    ShlAssign,
    /// The bitwise right-shift assignment operator, `>>=`.
    ShrAssign,
    /// The bitwise AND assignment operator, `&=`.
    AndAssign,
    /// The bitwise OR assignment operator, `|=`.
    OrAssign,
    /// The bitwise XOR assignment operator, `^=`.
    XorAssign,
    /// The assignment by sum operator, `+=`.
    AddAssign,
    /// The assignment by difference operator, `-=`.
    SubAssign,
    /// The assignment by product operator, `*=`.
    MulAssign,
    /// The assignment by division operator, `/=`.
    DivAssign,
    /// The assignment by modulo operator, `%=`.
    ModAssign,
}

///  A variable assignment.
#[derive(Debug, PartialEq)]

pub struct Assignment<'a> {
    /// The identifier being assigned to.
    pub ident: Node<'a, Ident>,
    /// The declared value.
    pub value: Node<'a, Expr<'a>>,
    /// The kind of assignment.
    pub kind: AssignmentKind,
}
