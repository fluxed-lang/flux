use chumsky::{Parser, prelude::Simple};
use fluxc_ast::{Node, Loop};
use fluxc_lexer::Token;

pub(crate) fn loop_expr() -> impl Parser<Token, Node<Loop>, Error = Simple<Token>> {
	todo!()
}
