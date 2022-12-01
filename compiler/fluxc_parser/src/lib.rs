//! The Flux parser, written using the `chumsky` library.

use chumsky::{prelude::Simple, recursive::recursive, select, Parser};
use fluxc_ast::{Expr, Literal, Node, Stmt};
use fluxc_lexer::{SpannedToken, Token};

fn parse() -> impl Parser<SpannedToken, Node<Stmt>, Error = Simple<Token>> {
    let literal = select! {
       (Token::LiteralInt(int), _) => Literal::Int(int),
       (Token::LiteralFloat(float), _) => Literal::Float(float),
       (Token::LiteralStr(str), _) => Literal::String(str),
       (Token::LiteralChar(c), _) => Literal::Char(c),
       (Token::LiteralBool(bool), _) => Literal::Bool(bool),
    }
    .map_with_span(|literal, span| Node::new(literal, span));

    let expr = recursive(|expr| {
        literal.map(Expr::Literal).map_with_span(|expr, span| Node::new(expr, span))
    });

    expr.map(Stmt::Expr).map_with_span(|stmt, span| Node::new(stmt, span))
}
