//! Contains the expression AST data structures.

use fluxc_ast::{Expr, Ident, Literal, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;
pub(crate) mod types;

use crate::{Context, Parse, Rule};

impl Parse for Expr {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        debug_assert_eq!(input.as_rule(), Rule::expr);
        // create node and unwrap inner rule
        let node = context.new_empty(input.as_span());
        let inner = input.into_inner().next().unwrap();
        // match rule
        let expr = match inner.as_rule() {
            Rule::literal => Expr::Literal(Literal::parse(inner, context)?),
            Rule::ident => Expr::Ident(Ident::parse(inner, context)?),
            _ => unreachable!(),
        };
        Ok(node.fill(expr))
    }
}
