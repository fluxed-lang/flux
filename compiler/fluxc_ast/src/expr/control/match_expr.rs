use crate::{Expr, Node, Literal};

/// A match expression.
pub struct Match {
    /// The expression being matched.
    pub expr: Box<Expr>,
    /// The list of cases being tested.
    pub cases: Vec<(Node<Literal>, Node<Expr>)>,
}
