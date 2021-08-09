use std::error::Error;
use std::str::FromStr;

/// A struct represnting a span of a string. The first paramteter is the start index of the span, 
/// and the second parameter is the end index of the span (inclusive).
#[derive(Debug, PartialEq)]
pub struct Span(
    /// The start index of the span.
    pub usize, 
    /// The end index of the span.
    pub usize
);

impl Span { 
    /// Returns true if this span includes another.
    pub const fn includes(&self, other: &Span) -> bool {
        self.0 < other.0 && self.1 > other.1
    }

    /// Returns true if this span overlaps with another.
    pub const fn overlaps(&self, other: &Span) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

#[cfg(test)]
mod span_test {
    use super::Span;
    #[test]
    fn span_test() {
        // a contains b, but does not contain c
        // a overlaps with both b and c.
        // c does not overlap with b.
        let a = Span(0, 10);
        let b = Span(3, 5);
        let c = Span(6, 11);

        assert!(a.includes(&b));
        assert!(!a.includes(&c));
        assert!(a.overlaps(&b));
        assert!(a.overlaps(&c));
        assert!(!b.overlaps(&c));
    }
}

#[derive(Debug, PartialEq)]
/// Enum representing the type of a literal.
pub enum LiteralKind {
    /// An integer literal (e.g. `1234`, `0x1234`, `0o1234`, `0b1001`).
    Int64(i64),
    Int32(i32),
    Int16(i16),
    Int8(i8),
    Uint64(u64),
    Uint32(u32),
    Uint16(u16),
    Uint8(u8),
    /// A floating-point literal (e.g. `1234.5`, `0x1234.5`, `0o1234.5`, `0b0110.1`).
    Float64(f64),
    Float32(f32),
    /// A string literal (e.g. `"hello"`, `"hello world"`).
    String(String),
    /// A character literal (e.g. `'a'`, `'\n'`).
    Char(char),
    /// A boolean literal (e.g. `true`, `false`).
    Bool(bool),
}

/// A literal value.
#[derive(Debug, PartialEq)]
pub struct Literal {
    /// The ID of this node in the AST.
    pub id: usize,
    /// The kind of literal.
    pub kind: LiteralKind,
    /// The span containing the literal.
    pub span: Span,
}

/// An argument to a function call.
#[derive(Debug, PartialEq)]
pub struct ParenArgument {
    /// The ID of the AST node.
    pub id: usize,
    /// The identifier representing the AST node.
    pub ident: usize
}

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
    Call(Vec<ParenArgument>),
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

#[derive(Debug, PartialEq)]
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

/// A declaration of a variable.
#[derive(Debug, PartialEq)]
pub struct Declaration {
    /// The identifier being declared.
    pub ident: Box<Stmt>,
    /// The mutability of the declared identifier.
    pub mutability: Mutability,
    /// The declared value.
    pub value: Box<Stmt>,
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
#[derive(Debug, PartialEq)]
pub enum Mutability {
    /// A mutable variable.
    Mutable,
    /// An immutable variable.
    Immutable,
    /// A constant. Unlike an immutable variable, the type of a constant must be defined at compile time, such
    /// that the size of the constant is known.
    Constant,
}

/// An identifier.
#[derive(Debug, PartialEq)]
pub struct Ident {
    /// The ID of this node in the AST.
    pub id: usize,
    /// The name of this node.
    pub name: String,
    /// The span corresponding to this node.
    pub span: Span
}

/// Enum of possible statement kinds.
#[derive(Debug, PartialEq)]
pub enum StmtKind {
    /// A block (e.g. `{ /* ... */ }`).
    Block(Box<Block>),
    /// A binary operation (e.g. `x = y * z + 1`.)
    BinOp(BinOpKind, Box<Stmt>, Box<Stmt>),
    /// A literal.
    Literal(Literal),
    /// An identifier.
    Ident(Ident),
    /// A declaration.
    Declaration(Declaration)
}

#[derive(Debug, PartialEq)]
pub struct Stmt {
    /// The ID of this node in the AST.
    pub id: usize,
    /// The kind of statement.
    pub kind: StmtKind,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    /// The list of statements in the block.
    pub stmts: Vec<Stmt>,
    /// The ID of this node in the AST.
    pub id: usize,
    /// The span of text that defines this block.   
    pub span: (usize, usize)
}

impl Block {
    /// Create a child block from this block. It will inherit the 
    fn create_child(&self, next_id: usize) -> Block {
        Block {
            stmts: vec![],
            id: next_id,
            span: self.span,
        }
    }
}

/// An external, imported module.
pub struct Module {
    /// The ID of the identifier representing this module.
    pub id: usize
}

/// A declared variable in the current context.
struct Var {
    /// The ID of the identifier representing this variable.
    pub ident: usize,
    /// The mutability of this variable.
    pub mutability: Mutability,

}

/// An AST context, in which variables are defined.
struct Context {
    /// The list of variables defined in this context.
    pub vars: Vec<Var>,
}

/// The root AST instance.
pub struct AST {
    /// The list of top-level statements in the AST.
    pub stmts: Vec<Stmt>,
    /// The list of external modules imported into this file.
    pub modules: Vec<Module>
}

impl AST {
    /// Create a new AST instance.
    pub fn new() -> AST {
        AST {
            stmts: vec![],
            modules: vec![],
        }
    }
}
