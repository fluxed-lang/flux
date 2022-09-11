use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Loop, Node};
use fluxc_lexer::Token;

use crate::{
    expr::{block_expr::block_expr, literal::literal_str},
    node,
};

pub(crate) fn loop_expr() -> impl Parser<Token, Node<Loop>, Error = Simple<Token>> + Clone {
    just(Token::KeywordLoop)
        .ignore_then(literal_str().or_not())
        .then(block_expr())
        .map(|(name, block)| Loop { name, block })
        .map_with_span(node)
}
