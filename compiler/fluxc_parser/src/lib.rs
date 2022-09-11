//! The Flux parser, written using the `chumsky` library.
use std::{ops::Range, sync::Mutex};

use chumsky::{prelude::Simple, select, Parser};
use fluxc_ast::{Ident, Node};
use fluxc_lexer::{Token, TokenStream};
use stmt::stmt;

pub(crate) mod expr;
pub(crate) mod stmt;

/// This method wraps `T` in a spanned AST node. For use with
/// `Parser::map_with_span`.
pub(crate) fn node<T>() -> impl Fn(T, Range<usize>) -> Node<T> {
    move |value, span| Node::new(value, span)
}

/// Parser combinator for [Ident].
pub(crate) fn ident() -> impl Parser<Token, Node<Ident>, Error = Simple<Token>> {
    select! {
        Token::Ident(ident) => ident
    }
    .map_with_span(node())
}

/// Parse a [TokenStream] into the AST.
pub fn parse(src: &str, input: TokenStream) {
    let parser = stmt().repeated();
}
