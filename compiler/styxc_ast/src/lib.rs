use std::error::Error;
use std::str::FromStr;

use styxc_types::Type;

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
pub enum Associativity {
    /// Left-to-right associativity.
    Ltr,
    /// Right-to-left associativity.
    Rtl,
}

/// Enum representing unary operator types.
///
/// Unary operators are operators that act on a single argument, such as `x++`, or `!x`.
pub enum UnOpKind {
    /// The suffix increment operator, `++`.
    SuffixIncr,
    /// The suffix decrement operator, `--`.
    SuffixDecr,
    /// The prefix increment operator, `++`.
    PrefixIncr,
    /// The prefix decrement operator, `--`.
    PrefixDecr,
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
    /// The call operator, `()`.
    Call(Vec<Node>),
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
            "++" => Err("cannot determine associativity of operator".into()),
            "--" => Err("cannot determine associativity of operator".into()),
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
            SuffixIncr | SuffixDecr | Index(_) => 1,
            _ => 2,
        }
    }

    /// Fetch the associativity of this unary operator.

    pub const fn associativity(&self) -> Associativity {
        use UnOpKind::*;
        match self {
            SuffixIncr | SuffixDecr | Index(_) => Associativity::Ltr,
            _ => Associativity::Rtl,
        }
    }
}

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
            "=" => Ok(Assign),
            "+=" => Ok(AddAssign),
            "-=" => Ok(SubAssign),
            "*=" => Ok(MulAssign),
            "%=" => Ok(ModAssign),
            "/=" => Ok(DivAssign),
             "&=" => Ok(AndAssign),
            "|=" => Ok(OrAssign),
            "^=" => Ok(XorAssign),
            "<<=" => Ok(ShlAssign),
            ">>=" => Ok(ShrAssign),
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
            BinOpKind::Assign => 14,
            // all other assignment operators have precedence 15.
            _ => 15,
        }
    }

    /// Fetch the associativity of this binary operator.
    pub const fn associativity(&self) -> Associativity {
        match self {
            BinOpKind::Assign
            | BinOpKind::AddAssign
            | BinOpKind::SubAssign
            | BinOpKind::MulAssign
            | BinOpKind::DivAssign
            | BinOpKind::ModAssign
            | BinOpKind::ShlAssign
            | BinOpKind::ShrAssign
            | BinOpKind::AndAssign
            | BinOpKind::XorAssign
            | BinOpKind::OrAssign => Associativity::Rtl,
            _ => Associativity::Ltr,
        }
    }
}

/// An enum representing variable mutability.
pub enum Mutability {
    /// A mutable variable.
    Mutable,
    /// An immutable variable.
    Immutable,
    /// A constant. Unlike an immutable variable, the type of a constant must be defined at compile time, such
    /// that the size of the constant is known.
    Constant,
}

/// An enum of all possible node types.
pub enum NodeKind {
    /// The root AST node.
    Root { children: Vec<Node> },

    /// An identifier.
    Ident {
        /// The name of the identifier.
        name: String,
        /// The ID of the identifier.
        id: usize,
    },

    /// The import statement.
    Import {
        /// The identifier the module is aliased to.
        alias: Option<Box<Node>>,
        /// The path or name of the module to import.
        target: String,
    },

    /// A variable reference.
    Variable {
        /// The identifier that represents this variable.
        ident: Box<Node>,
        /// The type of this variable.
        ty: Type,
        /// The mutability of this variable.
        mutability: Mutability,
    },

    /// A binary operation.
    BinOp {
        /// The kind of binary operation.
        kind: BinOpKind,
        /// The left operand.
        lhs: Box<Node>,
        /// The right operand.
        rhs: Box<Node>,
    },

    /// A block of code, `{ /* ... */ }`.
    Block {
        /// The list of statements in the block.
        children: Vec<Node>,
    },
}

/// A struct representing a node in the AST tree.
pub struct Node {
    /// The ID of this node in the tree.
    pub id: usize,
    /// The kind of this node.
    pub kind: NodeKind,
}
