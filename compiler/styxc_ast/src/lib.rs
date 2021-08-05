use crate::types::Type;

pub mod func;
pub mod types;
pub mod var;

pub enum NodeType {
    /// Represents a literal type. The second argument is the type of the literal.
    Literal(String, Box<Type>),

    /// Represents an identifier. This could be a variable, class, or function name.
    Ident(Ident),

    /// Represents a declaration.
    Declare(String, Box<Type>, Box<NodeType>),

    /// Represents a constant declaration.
    DeclareConst(String, Box<Type>, Box<NodeType>),

    /// Represents an assignment.
    Assign(String, Box<NodeType>),

    /// Represents a primitive type.
    Type(Box<Type>),

    /// Represents a binary equality expression.
    Eq(Box<NodeType>, Box<NodeType>),

    /// Represents a binary inequality expression.
    Ne(Box<NodeType>, Box<NodeType>),

    /// Represents a binary less-than expression.
    Lt(Box<NodeType>, Box<NodeType>),

    /// Represents a binary less-than-or-equal expression.
    Le(Box<NodeType>, Box<NodeType>),

    /// Represents a binary greater-than expression.
    Gt(Box<NodeType>, Box<NodeType>),

    /// Represents a binary greater-than-or-equal expression.
    Ge(Box<NodeType>, Box<NodeType>),

    /// Represents a binary addition expression.
    Add(Box<NodeType>, Box<NodeType>),

    /// Represents a binary subtraction expression.
    Sub(Box<NodeType>, Box<NodeType>),

    /// Represents a binary multiplication expression.
    Mul(Box<NodeType>, Box<NodeType>),

    /// Represents a binary division expression.
    Div(Box<NodeType>, Box<NodeType>),

    /// Represents an if statement. The first argument is the condition expression,
    /// the second argument is the statements to execute if this block is true.
    If(Box<NodeType>, Block),

    /// Represents an if-else statement. The first argument is the condition expression,
    /// the second argument is a vector of statements to execute if the condition is true,
    /// and the third s a vector of statements to execute if the condition expression is false.
    IfElse(Box<NodeType>, Block, Block),

    /// Represents a loop block.
    Loop(Box<Option<NodeType>>, Block),

    /// Represents a for block.
    /// for (expr; expr; expr) {}
    For(Box<NodeType>, Box<NodeType>, Box<NodeType>, Block),

    /// Represents a function declaration expression.
    FuncDeclare(FuncDeclare),

    /// Represents a function call.
    Call(String, Vec<NodeType>),

    /// Represents a top-level import.
    Import(String, String),
}

/// An enum of binary operation types.
pub enum BinOpKind {
    /// Represents the "+" operator.
    Add,
    /// Represents the "-" operator.
    Sub,
    /// Represents the "*" operator.
    Mul,
    /// Represents the "/" operator.
    Div,
    /// Represents the "%" operator.
    Mod,
    /// Represents the "==" operator.
    Eq,
    /// Represents the "<" operator.
    Lt,
    /// Represents the ">" operator.
    Gt,
    /// Represents the "<=" operator.
    Le,
    /// Represents the ">=" operator.
    Ge,
    /// Represents the ">>" operator.
    Shr,
    /// Represents the "<<" operator.
    Shl,
}

impl BinOpKind {
    /// Convert this operator to its string representation.
    pub fn to_string(&self) -> &'static str {
        use BinOpKind::*;
        match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Mod => "%",
            Eq => "==",
            Lt => "<",
            Gt => ">",
            Le => "<=",
            Ge => ">=",
            Shr => ">>",
            Shl => "<<",
        }
    }

    /// Test if this operation is a comparison operator.
    pub fn is_comparison(&self) -> bool {
        use BinOpKind::*;
        match *self {
            Eq | Lt | Gt | Le | Ge => true,
            Add | Sub | Mul | Div | Mod | Shr | Shl => false,
        }
    }
}

/// An enum of unary operation types.
pub enum UnOpKind {
    /// Represents the "!" operator.
    Not,
    /// Represents the "-" operator.
    Neg,
}

impl UnOpKind {
    /// Convert this operator into its string representation.
    pub fn to_string(&self) -> &'static str {
        use UnOpKind::*;
        match *self {
            Not => "!",
            Neg => "-",
        }
    }
}

/// Represents a node in the AST tree.
pub struct Node {}

/// Represents an identifier.
#[derive(Debug, Clone)]
pub struct Ident {
    /// The ID of this identifier in the AST.
    pub id: i64,
    /// The raw string name of this identifier.
    pub val: String,
}

/// An enum of mutability state.
/// e.g. a variable is mutable but a constant is not.
#[derive(Debug, Clone)]
pub enum Mutability {
    /// Represents an immutable state.
    Not,
    /// Represents a mutable state.
    Mut,
}

/// Represents a variable.
pub struct Variable {
    /// The identifier of this variable.
    pub ident: Ident,
    /// The mutability status of this variable.
    pub mutable: Mutability,
    /// The type of this variable.
    pub ty: Type,
}

/// Represents a block.
pub struct Block {
    /// The parent block.
    pub parent: Option<Box<Block>>,
    /// Expressions contained within this block.
    pub exprs: Vec<NodeType>,
}

/// Represents a function paretheses argument.
pub struct ParenArgument {
    /// The identifier of this paramenter.
    pub name: Ident,
    /// The type of this parameter.
    pub ty: Type,
}

/// Represents a function declaration.
pub struct FuncDeclare {
    /// The identifier of this function.
    ident: Ident,
    /// The method block of the function.
    method: Block,
    /// The parentheses arguments given to this function.
    args: Vec<ParenArgument>,
    /// The return type of the function.
    ret_ty: Type,
}
