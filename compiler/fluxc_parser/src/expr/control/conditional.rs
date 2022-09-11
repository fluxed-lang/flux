use chumsky::{Parser, prelude::Simple};
use fluxc_ast::{Node, Conditional};
use fluxc_lexer::Token;

pub(crate) fn conditional() -> impl Parser<Token, Node<Conditional>, Error = Simple<Token>> {
	todo!()
}
