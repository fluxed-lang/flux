use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Conditional, IfStmt, Node};
use fluxc_lexer::Token;

use crate::{expr::block_expr::block_expr, node, Parsers};

pub(crate) fn conditional<'a>(
    parsers: &'a Parsers<'a>,
) -> impl Parser<Token, Node<Conditional>, Error = Simple<Token>> + Clone + 'a {
    let if_stmt = just(Token::KeywordIf)
        .ignore_then(&parsers.expr)
        .then(block_expr(&parsers))
        .map(|(condition, block)| IfStmt { condition: Box::new(condition), block })
        .map_with_span(node);

    let else_if_stmt = just(Token::KeywordElse).ignore_then(if_stmt.clone());
    let else_stmt = just(Token::KeywordElse).ignore_then(block_expr(&parsers));

    if_stmt
        .then(else_if_stmt.repeated())
        .then(else_stmt.or_not())
        .map(|((if_stmt, else_ifs), else_stmt)| Conditional { if_stmt, else_ifs, else_stmt })
        .map_with_span(node)
}
