//! The Flux parser, written using the `chumsky` library.

use chumsky::{Parser, prelude::Simple};
use fluxc_ast::Literal;
use fluxc_lexer::Token;

trait Parse: Sized {
	fn combinator<P: Parser<Token, Self, Error = Simple<Token>>>() -> P;
}

impl Parse for Literal {
    fn combinator<P: Parser<Token, Self, Error = Simple<Token>>>() -> P {
        todo!()
    }
}
