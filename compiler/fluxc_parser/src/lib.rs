//! The Flux parser, written using the `chumsky` library.
use std::ops::Range;

use chumsky::{prelude::Simple, select, Parser, Stream};
use fluxc_ast::{Ident, Node, AST};
use fluxc_lexer::{Token, TokenStream};
use stmt::stmt;

pub(crate) mod expr;
pub(crate) mod stmt;

/// This method wraps `T` in a spanned AST node. For use with
/// `Parser::map_with_span`.
pub(crate) fn node<T>(value: T, span: Range<usize>) -> Node<T> {
    Node::new(value, span)
}

/// Parser combinator for [Ident].
pub(crate) fn ident() -> impl Parser<Token, Node<Ident>, Error = Simple<Token>> + Clone {
    select! {
        Token::Ident(ident) => ident
    }
    .map_with_span(node)
}

/// Parse a [TokenStream] into the AST.
pub fn parse(src: &str, input: TokenStream) -> Result<AST, Vec<Simple<Token>>> {
    let parser = stmt().repeated().map(|stmts| AST { stmts });
    parser.parse(Stream::from_iter(src.len()..src.len() + 1, input.into_iter()))
}
