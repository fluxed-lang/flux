use fluxc_ast::{Block, Expr, Node, While};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for While {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {
        debug_assert_eq!(input.as_rule(), Rule::while_stmt);
        let node = ctx.new_empty(input.as_span());
        let mut inner = input.into_inner();
        Ok(node.fill(While {
            condition: Box::new(Expr::parse(inner.next().unwrap(), ctx)?),
            block: Block::parse(inner.next().unwrap(), ctx)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Block, Expr, Literal, Node, Stmt, While};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_empty_while_stmt() {
        let mut context = Context::default();
        // while x {}
        let expected = Node {
            id: 0,
            span: Span::new(0, 9),
            value: While {
                condition: Box::new(Node {
                    id: 1,
                    span: Span::new(6, 6),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: Span::new(6, 6),
                        value: "x".to_string(),
                    }),
                }),
                block: Node { id: 3, span: Span::new(8, 9), value: Block { stmts: vec![] } },
            },
        };
        let actual = While::parse(
            FluxParser::parse(Rule::while_stmt, "while x {}").unwrap().next().unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_while_stmt() {
        let mut context = Context::default();
        // while x { "hello world!" }
        let expected = Node {
            id: 0,
            span: Span::new(0, 25),
            value: While {
                condition: Box::new(Node {
                    id: 1,
                    span: Span::new(6, 6),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: Span::new(6, 6),
                        value: "x".to_string(),
                    }),
                }),
                block: Node {
                    id: 3,
                    span: Span::new(8, 25),
                    value: Block {
                        stmts: vec![Node {
                            id: 4,
                            span: Span::new(10, 24),
                            value: Stmt::Expr(Node {
                                id: 5,
                                span: Span::new(10, 23),
                                value: Expr::Literal(Node {
                                    id: 6,
                                    span: Span::new(10, 23),
                                    value: Literal::String("hello world!".to_string()),
                                }),
                            }),
                        }],
                    },
                },
            },
        };
        let actual = While::parse(
            FluxParser::parse(Rule::while_stmt, "while x { \"hello world!\" }")
                .unwrap()
                .next()
                .unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }
}
