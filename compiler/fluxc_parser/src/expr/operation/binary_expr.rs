use chumsky::{Parser, prelude::Simple};
use fluxc_ast::{Node, BinaryExpr};
use fluxc_lexer::Token;

pub(crate) fn binary_expr() -> impl Parser<Token, Node<BinaryExpr>, Error = Simple<Token>> {
	todo!()
}
