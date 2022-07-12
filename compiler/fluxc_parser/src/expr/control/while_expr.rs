use fluxc_ast::{Block, Expr, While};
use pest::iterators::Pair;

use crate::{Context, PResult, Parse, Rule};

impl Parse for While {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
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

    use crate::{sealed::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_empty_while_stmt() {
        let mut context = Context::from_str("while x {}");
        let root = Span::from_str("while x {}");
        // while x {}
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 10),
            value: While {
                condition: Box::new(Node {
                    id: 1,
                    span: root.restrict_range(6, 7),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: root.restrict_range(6, 7),
                        value: "x".to_string(),
                    }),
                }),
                block: Node {
                    id: 3,
                    span: root.restrict_range(8, 10),
                    value: Block { stmts: vec![] },
                },
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
        let mut context = Context::from_str("while x { \"hello world!\" }");
        let root = Span::from_str("while x { \"hello world!\" }");
        // while x { "hello world!" }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 26),
            value: While {
                condition: Box::new(Node {
                    id: 1,
                    span: root.restrict_range(6, 7),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: root.restrict_range(6, 7),
                        value: "x".to_string(),
                    }),
                }),
                block: Node {
                    id: 3,
                    span: root.restrict_range(8, 26),
                    value: Block {
                        stmts: vec![Node {
                            id: 4,
                            span: root.restrict_range(10, 25),
                            value: Stmt::Expr(Node {
                                id: 5,
                                span: root.restrict_range(10, 24),
                                value: Expr::Literal(Node {
                                    id: 6,
                                    span: root.restrict_range(10, 24),
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
