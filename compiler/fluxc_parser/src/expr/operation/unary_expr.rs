use chumsky::{prelude::Simple, primitive::choice, select, Parser};
use fluxc_ast::{Node, UnaryExpr, UnaryOp};
use fluxc_lexer::Token;

use crate::{expr::expr, node};

pub(crate) fn unary_expr() -> impl Parser<Token, Node<UnaryExpr>, Error = Simple<Token>> + Clone {
    let suffix = expr()
        .then(select! {
            Token::TokenIncrement => UnaryOp::Increment,
            Token::TokenDecrement => UnaryOp::Decrement,
        })
        .map(|(expr, kind)| UnaryExpr { expr: Box::new(expr), kind })
        .map_with_span(node);

    let prefix = expr()
        .then(select! {
            Token::TokenAnd => UnaryOp::Reference,
            Token::TokenStar => UnaryOp::Dereference
        })
        .map(|(expr, kind)| UnaryExpr { expr: Box::new(expr), kind })
        .map_with_span(node);

    choice((suffix, prefix))
}
