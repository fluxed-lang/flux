use fluxc_span::Span;
use thiserror::Error;

use crate::Expr;

/// Wrapper around a generic type `T` that provides an AST ID, and a span.
///
/// This struct is used to wrap AST nodes with their ID, and position in the
/// source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    /// The ID of this node in the AST.
    pub id: usize,
    /// The span of the source code that this node represents.
    pub span: Span,
    /// The inner value held by this AST node.
    pub value: T,
}

impl<T> Node<T> {
    /// Create a new node.
    pub fn new(id: usize, span: Span, value: T) -> Self {
        Self { id, span, value }
    }
    /// Create an empty node with no value.
    pub fn empty(id: usize, span: Span) -> Node<()> {
        Node { id, span, value: () }
    }
}

impl<T: Clone> Node<T> {
    /// This method clones the inner value of the node and returns it.
    pub fn clone_inner(&self) -> T {
        self.value.clone()
    }
}

// generic implemetation of typed for all nodes
impl Node<()> {
    /// Hydrate this node with the given value.
    pub fn fill<T>(self, value: T) -> Node<T> {
        Node { id: self.id, span: self.span, value }
    }
}

#[derive(Debug, Error)]
enum AstCastError {
	/// The node is not of the expected type.
	#[error("Node is not of the expected type")]
	WrongType,
	/// The node is not of the expected variant.
	#[error("Node is not of the expected variant")]
	WrongVariant,
}

trait TryIntoAstType {
	/// Try to cast this node into an expression.
	fn try_into_expr(self) -> Result<Node<Expr>, AstCastError>;
}
