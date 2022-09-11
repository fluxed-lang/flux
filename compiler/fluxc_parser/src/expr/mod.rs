use chumsky::{prelude::Simple, primitive::choice, recursive::recursive, Parser};
use fluxc_ast::{Expr, Node};
use fluxc_lexer::Token;

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;

use self::literal::literal;
use crate::{ident, node};

/// Parser combinator for [Expr].
pub(crate) fn expr() -> impl Parser<Token, Node<Expr>, Error = Simple<Token>> {
    recursive(|expr| {
        let literal = literal().map(Expr::Literal);
        let ident = ident().map(Expr::Ident);
        choice((literal, ident))
    })
    .map_with_span(node)
    .boxed()
    .labelled("expression")
}
