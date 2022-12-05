use crate::{Expr, Node};

/// A match expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// The expression being matched.
    pub expr: Box<Node<Expr>>,
    /// The list of cases being tested.
    pub branches: Vec<Node<MatchBranch>>,
}
/// A match branch case.
#[derive(Debug, Clone, PartialEq)]
pub struct MatchBranch {
    /// The pattern being matched.
    pub pattern: Node<Expr>,
    /// The value to return if the pattern is matched.
    pub value: Node<Expr>,
}
