use crate::types::Type;

#[derive(Debug, Clone)]
pub enum Expr {
    /// Represents a literal type. The second argument is the type of the literal.
    Literal(String, Box<Type>),

    /// Represents an identifier. This could be a variable, class, or function name.
    Identifier(String),

    /// Represents a declaration.
    Declare(String, Box<Type>, Box<Expr>),

    /// Represents a constant declaration.
    DeclareConst(String, Box<Type>, Box<Expr>),

    /// Represents an assignment.
    Assign(String, Box<Expr>),

    /// Represents a primitive type.
    Type(Box<Type>),

    /// Represents a binary equality expression.
    Eq(Box<Expr>, Box<Expr>),

    /// Represents a binary inequality expression.
    Ne(Box<Expr>, Box<Expr>),

    /// Represents a binary less-than expression.
    Lt(Box<Expr>, Box<Expr>),

    /// Represents a binary less-than-or-equal expression.
    Le(Box<Expr>, Box<Expr>),

    /// Represents a binary greater-than expression.
    Gt(Box<Expr>, Box<Expr>),

    /// Represents a binary greater-than-or-equal expression.
    Ge(Box<Expr>, Box<Expr>),

    /// Represents a binary addition expression.
    Add(Box<Expr>, Box<Expr>),

    /// Represents a binary subtraction expression.
    Sub(Box<Expr>, Box<Expr>),

    /// Represents a binary multiplication expression.
    Mul(Box<Expr>, Box<Expr>),

    /// Represents a binary division expression.
    Div(Box<Expr>, Box<Expr>),

    /// Represents an if statement. The first argument is the condition expression,
    /// the second argument is the statements to execute if this block is true.
    If(Box<Expr>, Vec<Expr>),

    /// Represents an if-else statement. The first argument is the condition expression,
    /// the second argument is a vector of statements to execute if the condition is true,
    /// and the third s a vector of statements to execute if the condition expression is false.
    IfElse(Box<Expr>, Vec<Expr>, Vec<Expr>),

    /// Represents a loop block.
    Loop(Box<Option<Expr>>, Vec<Expr>),

    /// Represents a for block.
    /// for (expr; expr; expr) {}
    For(Box<Expr>, Box<Expr>, Box<Expr>, Vec<Expr>),

    /// Represents a function declaration expression.
    Function(String, Vec<String>, String, Vec<Expr>),

    /// Represents a function call.
    Call(String, Vec<Expr>),

    /// Represents a top-level import.
    Import(String, String),
}
