//! Contains the expression AST data structures.

use fluxc_ast::{Expr, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;

use crate::{Context, Parse, Rule};

impl Parse for Expr {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
