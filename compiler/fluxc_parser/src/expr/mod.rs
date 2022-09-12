use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    recursive::{recursive, Recursive},
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
use crate::{ident, node, Parsers};

/// Type alias for combinators requiring the use of the `expr` parser.
pub(crate) type ExprParser<'a> = Recursive<'a, Token, Node<Expr>, Simple<Token>>;

/// Parser combinator for [Expr].
pub(crate) fn expr<'a>(parsers: &'a Parsers<'a>) {
    let nested_expr = parsers
        .expr
        .delimited_by(just(Token::TokenParenthesisLeft), just(Token::TokenParenthesisRight));
    let conditional = conditional(&parsers).map(Expr::Conditional);
    let match_expr = match_expr(&parsers).map(Expr::Match);
    let binary_expr = binary_expr(&parsers).map(Expr::BinaryExpr);
    let unary_expr = unary_expr(&parsers).map(Expr::UnaryExpr);
    let literal = literal().map(Expr::Literal);
    let ident = ident().map(Expr::Ident);

    let expr =
        choice((nested_expr, conditional, match_expr, binary_expr, unary_expr, literal, ident))
            .map_with_span(node);

    parsers.expr.define(expr);
}
