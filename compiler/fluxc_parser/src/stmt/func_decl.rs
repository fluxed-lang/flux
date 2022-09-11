use chumsky::{
    prelude::Simple,
    primitive::{choice, just},
    Parser,
};
use fluxc_ast::{FuncDecl, FuncParam, Node};
use fluxc_lexer::Token;

use crate::{expr::block_expr::block_expr, ident, node};

pub(crate) fn func_param() -> impl Parser<Token, Node<FuncParam>, Error = Simple<Token>> + Clone {
    ident()
        .then_ignore(just(Token::TokenColon))
        .then(ident())
        .map(|(ident, ty)| FuncParam { ident, ty })
        .map_with_span(node)
}

pub(crate) fn func_decl() -> impl Parser<Token, Node<FuncDecl>, Error = Simple<Token>> + Clone {
    let params = func_param().separated_by(just(Token::TokenComma));

    let ret_ty = just(Token::TokenArrow).ignore_then(ident());
    let inner_func =
        ident()
            .then(params.delimited_by(
                just(Token::TokenParenthesisLeft),
                just(Token::TokenParenthesisRight),
            ))
            .then(ret_ty.or_not());

    let extern_func = just(Token::KeywordExtern)
        .ignore_then(inner_func.clone())
        .map(|((ident, params), ret_ty)| FuncDecl::External { ident, params, ret_ty });

    let local_func = inner_func
        .clone()
        .then(block_expr())
        .map(|(((ident, params), ret_ty), body)| FuncDecl::Local { ident, params, ret_ty, body });

    let exported_func = just(Token::KeywordExport)
        .ignore_then(inner_func.clone())
        .then(block_expr())
        .map(|(((ident, params), ret_ty), body)| FuncDecl::Export { ident, params, ret_ty, body });

    choice((extern_func, exported_func, local_func)).map_with_span(node)
}
