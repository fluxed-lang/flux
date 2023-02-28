//! The Flux parser, written using the `chumsky` library.

use chumsky::{prelude::*, Stream};
use fluxc_ast::{
    BinaryExpr, BinaryOp, Block, Conditional, Declaration, Expr, FuncCall, FuncDecl, FuncParam,
    IfStmt, Intersection, Literal, Loop, Mutability, Node, Operation, Primitive, Stmt,
    TypeDeclaration, TypeExpr, Union, AST,
};
use fluxc_lexer::{Token, TokenStream};

fn parser() -> impl Parser<Token, AST, Error = Simple<Token>> {
    // ident parser
    let raw_ident = select! {
        Token::Ident(ident) => ident
    };
    let ident = raw_ident.map_with_span(Node::new);

    // literals
    let literal = select! {
       Token::LiteralInt(int) => Literal::Int(int),
       Token::LiteralFloat(float) => Literal::Float(f64::from_be_bytes(float)),
       Token::LiteralStr(str) => Literal::String(str),
       Token::LiteralChar(c) => Literal::Char(c),
       Token::LiteralBool(bool) => Literal::Bool(bool),
       Token::LiteralUnit => Literal::Unit
    }
    .map_with_span(Node::new)
    .labelled("literal");

    let type_expr = recursive::<_, TypeExpr, _, _, Simple<Token>>(|ty_expr| {
        let type_literal = select! {
            Token::LiteralInt(int) => Primitive::IntLiteral(int),
            Token::LiteralFloat(float) => Primitive::FloatLiteral(f64::from_be_bytes(float)),
            Token::LiteralStr(str) => Primitive::StringLiteral(str),
            Token::LiteralChar(c) => Primitive::CharLiteral(c),
            Token::LiteralBool(bool) => match bool {
                true => Primitive::True,
                false => Primitive::False
            },
            Token::Ident(ident) => Primitive::Ref(ident),
            Token::LiteralUnit => Primitive::Unit
        }
        .map(TypeExpr::Primitive)
        .labelled("primitive type");

        let atom = type_literal.or(ty_expr
            .delimited_by(just(Token::TokenParenthesisLeft), just(Token::TokenParenthesisRight)));

        let intersection = atom
            .clone()
            .then(just(Token::TokenAnd).then(atom.clone()).repeated())
            .foldl(|lhs, (_, rhs)| {
                TypeExpr::Operation(Operation::Intersection(Intersection {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }))
            });

        let union = intersection
            .clone()
            .then(just(Token::TokenOr).then(intersection.clone()).repeated())
            .foldl(|lhs, (_, rhs)| {
                TypeExpr::Operation(Operation::Union(Union {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }))
            });

        let op = union;

        type_literal.or(op)
    })
    .labelled("type expression");

    // recursive stmt declaration
    let stmt = recursive::<_, Node<Stmt>, _, _, _>(|stmt| {
        let type_decl = just(Token::KeywordType)
            .ignore_then(ident)
            .then_ignore(just(Token::TokenAssign))
            .then(type_expr.clone().map_with_span(Node::new))
            .map(|(ident, value)| TypeDeclaration { ident, value })
            .map_with_span(Node::new)
            .labelled("type declaration");

        let block = stmt
            .repeated()
            .delimited_by(just(Token::TokenBraceLeft), just(Token::TokenBraceRight))
            .map(|stmts| Block { stmts })
            .map_with_span(Node::new)
            .labelled("block");

        let expr = recursive::<_, Node<Expr>, _, _, _>(|expr| {
            // conditionals
            let if_stmt = just(Token::KeywordIf)
                .ignore_then(expr.clone())
                .then(block.clone())
                .map(|(condition, block)| IfStmt { block, condition: Box::new(condition) })
                .map_with_span(Node::new)
                .labelled("if statement");

            let else_if_stmt = just(Token::KeywordElse).ignore_then(if_stmt.clone()).labelled("else-if statement");
            let else_stmt = just(Token::KeywordElse).ignore_then(block.clone()).labelled("else statement");

            let conditional = if_stmt
                .then(else_if_stmt.repeated())
                .then(else_stmt.or_not())
                .map(|((if_stmt, else_ifs), else_stmt)| Conditional {
                    if_stmt,
                    else_ifs,
                    else_stmt,
                })
                .map_with_span(Node::new)
                .labelled("conditional");

            let loop_expr = just(Token::KeywordLoop)
                .ignore_then(block.clone())
                .map(|block| Loop { block, name: None })
                .map_with_span(Node::new)
                .labelled("loop");

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
            let assign =
                sum.clone().then(op.then(sum.clone()).repeated()).foldl(|lhs, (kind, rhs)| {
                    let span = lhs.span.start..rhs.span.end;
                    Node::new(
                        Expr::BinaryExpr(Node::new(
                            BinaryExpr { lhs: Box::new(lhs), rhs: Box::new(rhs), kind },
                            span.clone(),
                        )),
                        span,
                    )
                });

            let bin_op = assign.labelled("binary operation");

            let func_call = ident
                .then(atom.separated_by(just(Token::TokenComma)))
                .map(|(ident, args)| FuncCall { ident, args })
                .map_with_span(Node::new).labelled("function call");

            choice((
                func_call.map(Expr::FuncCall),
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
            .map_with_span(Node::new)
            .labelled("declaration");

        let return_stmt = just(Token::KeywordReturn)
            .ignore_then(expr.clone())
            .map(Stmt::Return)
            .labelled("return statement");

        // function declarations
        let func_decl_param = ident
            .then_ignore(just(Token::TokenColon))
            .then(type_expr.clone().map_with_span(Node::new))
            .map(|(ident, ty)| FuncParam { ident, ty })
            .labelled("parameter");

        let func_decl_params = (func_decl_param.clone().map_with_span(Node::new))
            .separated_by(just(Token::TokenComma));

        let extern_func_decl = just(Token::KeywordExtern)
            .ignore_then(ident)
            .then(func_decl_params.or_not())
            .then_ignore(just(Token::TokenArrow))
            .then(type_expr.clone().map_with_span(Node::new).or_not())
            .map(|((ident, params), ret_ty)| FuncDecl::External {
                ident,
                params: params.unwrap_or_default(),
                ret_ty,
            })
            .labelled("external function declaration");

        let func_decl = (extern_func_decl).map_with_span(Node::new);

        choice::<_, Simple<Token>>((
            declaration.map(Stmt::Declaration),
            type_decl.map(Stmt::TypeDeclaration),
            func_decl.map(Stmt::FuncDecl),
            return_stmt,
            expr.map(Stmt::Expr),
        ))
        .map_with_span(Node::new)
    });

    stmt.repeated().then_ignore(end()).map(|stmts| AST { stmts })
}

#[tracing::instrument]
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
