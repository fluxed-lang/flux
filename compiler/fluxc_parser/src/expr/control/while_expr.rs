use chumsky::{Parser, prelude::Simple};
use fluxc_ast::{Node, While};
use fluxc_lexer::Token;

pub(crate) fn while_expr() -> impl Parser<Token, Node<While>, Error = Simple<Token>> {
	todo!()
}
