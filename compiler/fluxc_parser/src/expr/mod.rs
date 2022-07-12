//! Contains the expression AST data structures.

use fluxc_ast::{Expr, Ident, Literal};

use pest::iterators::Pair;

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;

use crate::{Context, PResult, Parse, Rule};

impl Parse for Expr {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context) -> PResult<Self> {
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

impl Parse for Ident {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context) -> PResult<Self> {
        Ok(context.new_node(input.as_span(), input.as_str().into()))
    }
}
