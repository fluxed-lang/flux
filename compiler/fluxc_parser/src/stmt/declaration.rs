use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Declaration, Mutability, Node};
use fluxc_lexer::Token;

use crate::{expr::expr, ident, node};

pub(crate) fn declaration() -> impl Parser<Token, Node<Declaration>, Error = Simple<Token>> + Clone
{
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
        .then(expr())
        .map(|((ident, mutability), value)| Declaration {
            explicit_ty: None,
            ident,
            mutability,
            value,
        })
        .map_with_span(node)
}
