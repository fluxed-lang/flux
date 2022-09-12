use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    Parser,
};
use fluxc_ast::{BinaryExpr, BinaryOp, Node};
use fluxc_lexer::Token;

use crate::{node, Parsers};

pub(crate) fn binary_expr<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<BinaryExpr>, Error = Simple<Token>> + Clone + 'a {
    let op = just(Token::TokenStar).to(BinaryOp::Mul).or(just(Token::TokenSlash).to(BinaryOp::Div));
    let product = parsers
        .expr
        .clone()
        .then(op.then(&parsers.expr))
        .map(|(lhs, (kind, rhs))| BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), kind })
        .map_with_span(node);

    let op =
        just(Token::TokenPlus).to(BinaryOp::Plus).or(just(Token::TokenMinus).to(BinaryOp::Minus));
    let sum = parsers
        .expr
        .clone()
        .then(op.then(&parsers.expr))
        .map(|(lhs, (kind, rhs))| BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), kind })
        .map_with_span(node);

    let op = just(Token::TokenEq).to(BinaryOp::Eq).or(just(Token::TokenNe).to(BinaryOp::Ne));

    let compare = parsers
        .expr
        .clone()
        .then(op.then(&parsers.expr))
        .map(|(lhs, (kind, rhs))| BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), kind })
        .map_with_span(node);

    choice((product, sum, compare))
}
