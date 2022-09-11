use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    recursive::recursive,
    Parser,
};
use fluxc_ast::{Expr, Node};
use fluxc_lexer::Token;

pub(crate) mod block_expr;
pub(crate) mod control;
pub(crate) mod literal;
pub(crate) mod operation;

use self::{
    control::{conditional::conditional, match_expr::match_expr},
    literal::literal,
    operation::{binary_expr::binary_expr, unary_expr::unary_expr},
};
use crate::{ident, node};

/// Parser combinator for [Expr].
pub(crate) fn expr() -> impl Parser<Token, Node<Expr>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let nested_expr = expr
            .delimited_by(just(Token::TokenParenthesisLeft), just(Token::TokenParenthesisRight));
        let conditional = conditional().map(Expr::Conditional);
        let match_expr = match_expr().map(Expr::Match);
        let binary_expr = binary_expr().map(Expr::BinaryExpr);
        let unary_expr = unary_expr().map(Expr::UnaryExpr);
        let literal = literal().map(Expr::Literal);
        let ident = ident().map(Expr::Ident);
        choice((nested_expr, conditional, match_expr, binary_expr, unary_expr, literal, ident))
    })
    .map_with_span(node)
    .labelled("expression")
}
