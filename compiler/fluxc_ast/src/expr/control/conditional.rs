use crate::{Block, Expr, Node};

/// A generic conditional expression.
#[derive(Debug, PartialEq)]
pub struct Conditional {
    /// The opening `if` statement of this conditional.
    pub if_stmt: Node<IfStmt>,
    /// A list of `else if` statements to test if the above conditional is not
    /// executed.
    pub else_ifs: Vec<Node<IfStmt>>,
    /// An optional `else` block to execute if all above conditionals are not
    /// executed.
    pub else_stmt: Option<Node<Block>>,
}

/// A conditional `if` statement.
#[derive(Debug, PartialEq)]
pub struct IfStmt {
    /// The condition to test.
    pub condition: Box<Node<Expr>>,
    /// The value to return if the condition is true.
    pub value: Node<Block>,
}
