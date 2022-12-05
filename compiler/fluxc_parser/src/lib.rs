//! The Flux parser, written using the `chumsky` library.

use chumsky::{prelude::*, Stream};
use fluxc_ast::{
    BinaryExpr, BinaryOp, Block, Conditional, Declaration, Expr, IfStmt, Literal, Loop, Mutability,
    Node, Stmt, AST,
};
use fluxc_lexer::{Token, TokenStream};

fn parser() -> impl Parser<Token, AST, Error = Simple<Token>> {
    // ident parser
    let ident = select! {
        Token::Ident(ident) => ident
    }
    .map_with_span(Node::new);

    // literals
    let literal = select! {
       Token::LiteralInt(int) => Literal::Int(int),
       Token::LiteralFloat(float) => Literal::Float(f64::from_be_bytes(float)),
       Token::LiteralStr(str) => Literal::String(str),
       Token::LiteralChar(c) => Literal::Char(c),
       Token::LiteralBool(bool) => Literal::Bool(bool),
    }
    .map_with_span(Node::new);

    // recursive stmt declaration
    let stmt = recursive::<_, Node<Stmt>, _, _, _>(|stmt| {
        let block = stmt
            .repeated()
            .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBraceRight))
            .map(|stmts| Block { stmts })
            .map_with_span(Node::new);

        let expr = recursive::<_, Node<Expr>, _, _, _>(|expr| {
            // conditionals
            let if_stmt = just(Token::KeywordIf)
                .ignore_then(expr.clone())
                .then(block.clone())
                .map(|(condition, block)| IfStmt { block, condition: Box::new(condition) })
                .map_with_span(Node::new);

            let else_if_stmt = just(Token::KeywordElse).ignore_then(if_stmt.clone());
            let else_stmt = just(Token::KeywordElse).ignore_then(block.clone());

            let conditional = if_stmt
                .then(else_if_stmt.repeated())
                .then(else_stmt.or_not())
                .map(|((if_stmt, else_ifs), else_stmt)| Conditional {
                    if_stmt,
                    else_ifs,
                    else_stmt,
                })
                .map_with_span(Node::new);

            let loop_expr = just(Token::KeywordLoop)
                .ignore_then(block.clone())
                .map(|block| Loop { block, name: None })
                .map_with_span(Node::new);

            // binary expr
            let atom =
                ident.map(Expr::Ident).or(literal.map(Expr::Literal)).map_with_span(Node::new).or(
                    expr.clone().delimited_by(
                        just(Token::TokenParenthesisLeft),
                        just(Token::TokenParenthesisRight),
                    ),
                );

            // sum operations
            let op = select! {
                Token::TokenPlus => BinaryOp::Plus,
                Token::TokenMinus => BinaryOp::Minus,
            };

            let sum =
                atom.clone().then(op.then(atom.clone()).repeated()).foldl(|lhs, (kind, rhs)| {
                    let span = lhs.span.start..rhs.span.end;
                    Node::new(
                        Expr::BinaryExpr(Node::new(
                            BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), kind },
                            span.clone(),
                        )),
                        span,
                    )
                });

            // assignment
            let op = select! {
				Token::TokenAssign => BinaryOp::Assign,
                Token::TokenPlusEq => BinaryOp::PlusEq,
                Token::TokenMinusEq => BinaryOp::MinusEq,
                Token::TokenMulEq => BinaryOp::MulEq,
                Token::TokenDivEq => BinaryOp::DivEq
            };
            let assign = sum
                .clone()
                .then(op.then(sum.clone()).repeated())
                .foldl(|lhs, (kind, rhs)| {
                    let span = lhs.span.start..rhs.span.end;
                    Node::new(
                        Expr::BinaryExpr(Node::new(
                            BinaryExpr {
                                lhs: Box::new(lhs),
                                rhs: Box::new(rhs),
                                kind,
                            },
                            span.clone(),
                        )),
                        span,
                    )
                });

            let bin_op = assign;

            choice((
                literal.map(Expr::Literal),
                block.map(Expr::Block),
                loop_expr.map(Expr::Loop),
                conditional.map(Expr::Conditional),
            ))
            .map_with_span(Node::new)
            .or(bin_op)
        });

        let declaration_idents = ident.then_ignore(just(Token::TokenComma)).repeated().chain(ident);

        let declaration = just(Token::KeywordLet)
            .ignore_then(declaration_idents)
            .then_ignore(just(Token::TokenAssign))
            .then(expr.clone())
            .map(|(idents, value)| Declaration {
                explicit_ty: None,
                ident: idents.into_iter().next().unwrap(),
                mutability: Mutability::Immutable,
                value,
            })
            .map_with_span(Node::new);

        choice::<_, Simple<Token>>((declaration.map(Stmt::Declaration), expr.map(Stmt::Expr)))
            .map_with_span(Node::new)
    });

    stmt.repeated().then_ignore(end()).map(|stmts| AST { stmts })
}

pub fn parse(input: TokenStream) -> Result<AST, Vec<Simple<Token>>> {
    // empty tokens
    if input.is_empty() {
        return Ok(AST { stmts: vec![] });
    }

    // compute eoi
    let start = input.first().expect("empty stream").start();
    let end = input.last().expect("empty stream").end();
    let eoi = start..end;

    parser().parse(Stream::from_iter(eoi, input.into_iter()))
}
