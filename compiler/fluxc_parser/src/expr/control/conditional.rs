use chumsky::{prelude::Simple, primitive::just, Parser};
use fluxc_ast::{Conditional, IfStmt, Node};
use fluxc_lexer::Token;

use crate::{
    expr::{block_expr::block_expr, expr},
    node,
};

pub(crate) fn conditional() -> impl Parser<Token, Node<Conditional>, Error = Simple<Token>> + Clone
{
    let if_stmt = just(Token::KeywordIf)
        .ignore_then(expr())
        .then(block_expr())
        .map(|(condition, block)| IfStmt { condition: Box::new(condition), block })
        .map_with_span(node);
    let else_if_stmt = just(Token::KeywordElse).ignore_then(if_stmt.clone());
    let else_stmt = just(Token::KeywordElse).ignore_then(block_expr());

    if_stmt
        .then(else_if_stmt.repeated())
        .then(else_stmt.or_not())
        .map(|((if_stmt, else_ifs), else_stmt)| Conditional { if_stmt, else_ifs, else_stmt })
        .map_with_span(node)
}
