use fluxc_ast::{Expr, Match, MatchBranch};
use pest::iterators::Pair;

use crate::{Context, PResult, Parse, Rule};

impl Parse for Match {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context) -> PResult<Self> {
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
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context) -> PResult<Self> {
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

    use crate::{sealed::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_empty_match_expr() {
        let mut context = Context::from_str("match x {}");
        let root = Span::from_str("match x {}");
        // match x {}
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 10),
            value: Match {
                expr: Box::new(Node {
                    id: 1,
                    span: root.restrict_range(6, 7),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: root.restrict_range(6, 7),
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
        let mut context = Context::from_str("match x { 1 -> 1, 2 -> 2 }");
        let root = Span::from_str("match x { 1 -> 1, 2 -> 2 }");
        // match x { 1 -> 1, 2 -> 2 }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 26),
            value: Match {
                expr: Box::new(Node {
                    id: 1,
                    span: root.restrict_range(6, 7),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: root.restrict_range(6, 7),
                        value: "x".to_string(),
                    }),
                }),
                cases: vec![
                    Node {
                        id: 3,
                        span: root.restrict_range(10, 16),
                        value: MatchBranch {
                            pattern: Node {
                                id: 4,
                                span: root.restrict_range(10, 11),
                                value: Expr::Literal(Node {
                                    id: 5,
                                    span: root.restrict_range(10, 11),
                                    value: Literal::Int(1),
                                }),
                            },
                            value: Node {
                                id: 6,
                                span: root.restrict_range(15, 16),
                                value: Expr::Literal(Node {
                                    id: 7,
                                    span: root.restrict_range(15, 16),
                                    value: Literal::Int(1),
                                }),
                            },
                        },
                    },
                    Node {
                        id: 8,
                        span: root.restrict_range(18, 24),
                        value: MatchBranch {
                            pattern: Node {
                                id: 9,
                                span: root.restrict_range(18, 19),
                                value: Expr::Literal(Node {
                                    id: 10,
                                    span: root.restrict_range(18, 19),
                                    value: Literal::Int(2),
                                }),
                            },
                            value: Node {
                                id: 11,
                                span: root.restrict_range(23, 24),
                                value: Expr::Literal(Node {
                                    id: 12,
                                    span: root.restrict_range(23, 24),
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
