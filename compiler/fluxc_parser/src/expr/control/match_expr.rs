use fluxc_ast::{Expr, Match, MatchBranch};
use pest::iterators::Pair;

use crate::{Context, Parse, Rule, PResult};

impl Parse for Match {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::match_expr);
        let node = context.new_empty(input.as_span());
        let mut inner = input.into_inner();
        Ok(node.fill(Match {
            expr: Box::new(Expr::parse(inner.next().unwrap(), context)?),
            cases: inner.map(|pair| MatchBranch::parse(pair, context)).collect::<Result<_, _>>()?,
        }))
    }
}

impl Parse for MatchBranch {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::match_branch);
        let node = context.new_empty(input.as_span());
        let mut inner = input.into_inner();
        Ok(node.fill(MatchBranch {
            pattern: Expr::parse(inner.next().unwrap(), context)?,
            value: Expr::parse(inner.last().unwrap(), context)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Expr, Literal, Match, MatchBranch, Node};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_empty_match_expr() {
        let mut context = Context::default();
        // match x {}
        let expected = Node {
            id: 0,
            span: Span::new(0, 9),
            value: Match {
                expr: Box::new(Node {
                    id: 1,
                    span: Span::new(6, 6),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: Span::new(6, 6),
                        value: "x".to_string(),
                    }),
                }),
                cases: vec![],
            },
        };
        let actual = Match::parse(
            FluxParser::parse(Rule::match_expr, "match x {}").unwrap().next().unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_match_expr() {
        let mut context = Context::default();
        // match x { 1 -> 1, 2 -> 2 }
        let expected = Node {
            id: 0,
            span: Span::new(0, 25),
            value: Match {
                expr: Box::new(Node {
                    id: 1,
                    span: Span::new(6, 6),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: Span::new(6, 6),
                        value: "x".to_string(),
                    }),
                }),
                cases: vec![
                    Node {
                        id: 3,
                        span: Span::new(10, 15),
                        value: MatchBranch {
                            pattern: Node {
                                id: 4,
                                span: Span::new(10, 10),
                                value: Expr::Literal(Node {
                                    id: 5,
                                    span: Span::new(10, 10),
                                    value: Literal::Int(1),
                                }),
                            },
                            value: Node {
                                id: 6,
                                span: Span::new(15, 15),
                                value: Expr::Literal(Node {
                                    id: 7,
                                    span: Span::new(15, 15),
                                    value: Literal::Int(1),
                                }),
                            },
                        },
                    },
                    Node {
                        id: 8,
                        span: Span::new(18, 23),
                        value: MatchBranch {
                            pattern: Node {
                                id: 9,
                                span: Span::new(18, 18),
                                value: Expr::Literal(Node {
                                    id: 10,
                                    span: Span::new(18, 18),
                                    value: Literal::Int(2),
                                }),
                            },
                            value: Node {
                                id: 11,
                                span: Span::new(23, 23),
                                value: Expr::Literal(Node {
                                    id: 12,
                                    span: Span::new(23, 23),
                                    value: Literal::Int(2),
                                }),
                            },
                        },
                    },
                ],
            },
        };
        let actual = Match::parse(
            FluxParser::parse(Rule::match_expr, "match x { 1 -> 1, 2 -> 2 }")
                .unwrap()
                .next()
                .unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }
}
