use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Declaration, Mutability, Node};
use fluxc_lexer::Token;

use crate::{ident, node, Parsers};

pub(crate) fn declaration<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<Declaration>, Error = Simple<Token>> + Clone + 'a {
    just(Token::KeywordLet)
        // ident
        .ignore_then(ident())
        // parse mutability
        .then(just(Token::KeywordMut).or_not().map(|token| match token {
            Some(_) => Mutability::Mutable,
            None => Mutability::Immutable,
        }))
        .then_ignore(just(Token::TokenEq))
        // value
        .then(&parsers.expr)
        .map(|((ident, mutability), value)| Declaration {
            explicit_ty: None,
            ident,
            mutability,
            value,
        })
        .map_with_span(node)
}
