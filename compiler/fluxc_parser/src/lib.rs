//! The Flux parser, written using the `chumsky` library.

use std::sync::Mutex;

use chumsky::{prelude::Simple, select, Parser, recursive::recursive};
use fluxc_ast::{Literal, Node, AST, Expr};
use fluxc_lexer::{Token, TokenPair};

trait Parse: Sized {
    fn combinator<P: Parser<TokenPair, Node<Self>, Error = Simple<TokenPair>>>(ctx: Ctx) -> P;
}

struct Ctx {
    next_id: Mutex<usize>,
}

impl Ctx {
    pub fn create_empty(&self) -> Node<()> {
        let mut id = self.next_id.lock().expect("failed to lock ctx id mutex");
        let node_id: usize = *id;
        id = id + 1;
        Node { id: node_id }
    }
}

impl Parse for Literal {
    fn combinator<P: Parser<TokenPair, Node<Self>, Error = Simple<TokenPair>>>(ctx: Ctx) -> P {
        todo!()
    }
}

fn literal(ctx: Ctx) -> impl Parser<TokenPair, Node<Literal>, Error = Simple<TokenPair>> {
    select! {
		Token::Str => Literal::String("".into())
	}.labelled("value")
}


fn parser() -> impl Parser<TokenPair, AST, Error = Simple<TokenPair>> {
	expr
}

fn expr() -> impl Parser<TokenPair, Node<Expr>, Error = Simple<TokenPair>> {
	literal().map_with_span(f)
}
