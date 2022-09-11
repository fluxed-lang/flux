//! Contains data structures for representing imports and exports.

use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Export, Import, ModuleSymbol, Node};
use fluxc_lexer::Token;

use crate::{expr::literal::literal_str, ident, node};

pub(crate) fn module_symbol() -> impl Parser<Token, Node<ModuleSymbol>, Error = Simple<Token>> {
    ident()
        .then(just(Token::KeywordAs).ignore_then(literal_str().map_with_span(node())).or_not())
        .map(|(name, alias)| ModuleSymbol { name, alias })
        .map_with_span(node())
}

pub(crate) fn import() -> impl Parser<Token, Node<Import>, Error = Simple<Token>> {
    let idents = module_symbol().separated_by(just(Token::TokenComma).ignored());
    idents
        .then(just(Token::KeywordFrom).ignore_then(literal_str()))
        .map(|(symbols, path)| Import { symbols, path })
        .map_with_span(node())
}

pub(crate) fn export() -> impl Parser<Token, Node<Export>, Error = Simple<Token>> {
    let idents = module_symbol().separated_by(just(Token::TokenComma).ignored());
    just(Token::KeywordExport)
        .ignore_then(idents)
        .then(just(Token::KeywordFrom).ignore_then(literal_str()))
        .map(|(symbols, path)| Export { symbols })
        .map_with_span(node())
}
