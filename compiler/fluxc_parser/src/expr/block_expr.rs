use std::rc::Rc;

use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::Block;
use fluxc_lexer::Token;

use crate::{node, stmt::stmt, Node};

pub(crate) fn block_expr() -> impl Parser<Token, Node<Block>, Error = Simple<Token>> {
    stmt()
        .repeated()
        .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBraceRight))
        .map(|stmts| Block { stmts })
        .map_with_span(node)
}
