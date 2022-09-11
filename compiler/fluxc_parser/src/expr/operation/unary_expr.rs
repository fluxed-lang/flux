use chumsky::{Parser, prelude::Simple};
use fluxc_ast::{UnaryExpr, Node};
use fluxc_lexer::Token;

pub(crate) fn unary_expr() -> impl Parser<Token, Node<UnaryExpr>, Error = Simple<Token>> {
	todo!()
}
