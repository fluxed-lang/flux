use crate::{Block, Expr, Ident, Literal, Node};

/// A generic conditional expression.
#[derive(Debug, PartialEq)]
pub struct Conditional {
    /// The opening `if` statement of this conditional.
    pub if_stmt: (Box<Expr>, Node<Block>),
    /// A list of `else if` statements to test if the above conditional is not executed.
    pub else_ifs: Vec<(Expr, Node<Block>)>,
    /// An optional `else` block to execute if all above conditionals are not executed.
    pub else_stmt: Option<Block>,
}

/// A match expression.
pub struct Match {
    /// The expression being matched.
    pub expr: Box<Expr>,
    /// The list of cases being tested.
    pub cases: Vec<(Node<Literal>, Node<Expr>)>,
}

#[derive(Debug, PartialEq)]
pub struct Loop {
    /// The name of this loop.
    pub name: Ident,
    /// The block owned by this loop.
    pub block: Node<Block>,
}

/// A `while` loop.
#[derive(Debug, PartialEq)]
pub struct While {
    /// The loop condition.
    pub condition: Box<Expr>,
    /// The block executed every time this loop is executed.
    pub block: Node<Block>,
}
