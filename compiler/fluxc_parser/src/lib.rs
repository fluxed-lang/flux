//! The Flux parser, written using the `chumsky` library.
use std::ops::Range;

use chumsky::{prelude::*, select, Stream};
use expr::expr;
use fluxc_ast::{Expr, Ident, Node, Stmt, AST};
use fluxc_lexer::{Token, TokenStream};
use stmt::stmt;

pub(crate) mod expr;
pub(crate) mod stmt;

#[derive(Clone)]
pub(crate) struct Parsers<'a> {
    expr: Recursive<'a, Token, Node<Expr>, Simple<Token>>,
    stmt: Recursive<'a, Token, Node<Stmt>, Simple<Token>>,
}

impl Parsers<'_> {
    fn new() -> Self {
        Parsers { expr: Recursive::declare(), stmt: Recursive::declare() }
    }
}

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
    let definitions = Parsers::new();
    stmt(&definitions);
    expr(&definitions);

    let parser = definitions.stmt.repeated().map(|stmts| AST { stmts });

    parser.parse(Stream::from_iter(src.len()..src.len() + 1, input.into_iter()))
}
