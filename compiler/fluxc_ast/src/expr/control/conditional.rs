use crate::{Expr, Block, Node};

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
