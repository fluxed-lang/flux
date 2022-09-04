use fluxc_ast::{Block, Conditional, Expr, IfStmt};
use pest::iterators::Pair;

use crate::{Context, PResult, Parse, Rule, util::IntoSpan};

impl Parse for Conditional {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::conditional_stmt);
        let node = ctx.new_empty(input.as_span());
        let mut inner = input.into_inner();
        // parse if stmt
        let if_stmt = IfStmt::parse(inner.next().unwrap(), ctx)?;
        // iterate over the rest of the statements
        let mut else_ifs = vec![];
        while inner.peek().map(|pair| pair.as_rule() == Rule::else_if_stmt).unwrap_or(false) {
            let else_if = IfStmt::parse(inner.next().unwrap(), ctx)?;
            else_ifs.push(else_if);
        }
        // if the else stmt exists, parse and append
        let else_stmt = match inner.next() {
            Some(pair) => {
                debug_assert_eq!(pair.as_rule(), Rule::else_stmt);
                let inner = pair.into_inner().next().unwrap();
                Some(Block::parse(inner, ctx)?)
            }
            None => None,
        };

        Ok(node.fill(Conditional { if_stmt, else_ifs, else_stmt }))
    }
}

impl Parse for IfStmt {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert!(input.as_rule() == Rule::if_stmt || input.as_rule() == Rule::else_if_stmt);
        let node = ctx.new_empty(input.as_span());
        // unwrap into inner
        let mut inner = input.into_inner();
        Ok(node.fill(IfStmt {
            // parse condition
            condition: Box::new(Expr::parse(inner.next().unwrap(), ctx)?),
            // parse value
            value: Block::parse(inner.next().unwrap(), ctx)?,
        }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Block, Conditional, Expr, IfStmt, Literal, Node, Stmt};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_single_if() {
        let mut context = Context::from_str("if true { 1 }");
        let root = Span::from_str("if true { 1 }");
        // if true { 1 }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 13),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: root.restrict_range(0, 13),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: root.restrict_range(3, 7),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: root.restrict_range(3, 7),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: root.restrict_range(8, 13),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: root.restrict_range(10, 12),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: root.restrict_range(10, 11),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: root.restrict_range(10, 11),
                                            value: Literal::Int(1),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                },
                else_ifs: vec![],
                else_stmt: None,
            },
        };
        let actual = Conditional::parse(
            FluxParser::parse(Rule::conditional_stmt, "if true { 1 }").unwrap().next().unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_if_else() {
        let mut context = Context::from_str("if true { 1 } else { 2 }");
        let root = Span::from_str("if true { 1 } else { 2 }");
        // if true { 1 } else { 2 }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 24),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: root.restrict_range(0, 13),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: root.restrict_range(3, 7),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: root.restrict_range(3, 7),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: root.restrict_range(8, 13),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: root.restrict_range(10, 12),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: root.restrict_range(10, 11),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: root.restrict_range(10, 11),
                                            value: Literal::Int(1),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                },
                else_ifs: vec![],
                else_stmt: Some(Node {
                    id: 8,
                    span: root.restrict_range(19, 24),
                    value: Block {
                        stmts: vec![Node {
                            id: 9,
                            span: root.restrict_range(21, 23),
                            value: Stmt::Expr(Node {
                                id: 10,
                                span: root.restrict_range(21, 22),
                                value: Expr::Literal(Node {
                                    id: 11,
                                    span: root.restrict_range(21, 22),
                                    value: Literal::Int(2),
                                }),
                            }),
                        }],
                    },
                }),
            },
        };
        let actual = Conditional::parse(
            FluxParser::parse(Rule::conditional_stmt, "if true { 1 } else { 2 }")
                .unwrap()
                .next()
                .unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_if_else_if() {
        let mut context = Context::from_str("if true { 1 } else if false { 2 }");
        let root = Span::from_str("if true { 1 } else if false { 2 }");
        // if true { 1 } else if false { 2 }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 33),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: root.restrict_range(0, 13),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: root.restrict_range(3, 7),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: root.restrict_range(3, 7),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: root.restrict_range(8, 13),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: root.restrict_range(10, 12),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: root.restrict_range(10, 11),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: root.restrict_range(10, 11),
                                            value: Literal::Int(1),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                },
                else_ifs: vec![Node {
                    id: 8,
                    span: root.restrict_range(14, 33),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 9,
                            span: root.restrict_range(22, 27),
                            value: Expr::Literal(Node {
                                id: 10,
                                span: root.restrict_range(22, 27),
                                value: Literal::Bool(false),
                            }),
                        }),
                        value: Node {
                            id: 11,
                            span: root.restrict_range(28, 33),
                            value: Block {
                                stmts: vec![Node {
                                    id: 12,
                                    span: root.restrict_range(30, 32),
                                    value: Stmt::Expr(Node {
                                        id: 13,
                                        span: root.restrict_range(30, 31),
                                        value: Expr::Literal(Node {
                                            id: 14,
                                            span: root.restrict_range(30, 31),
                                            value: Literal::Int(2),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                }],
                else_stmt: None,
            },
        };
        let actual = Conditional::parse(
            FluxParser::parse(Rule::conditional_stmt, "if true { 1 } else if false { 2 }")
                .unwrap()
                .next()
                .unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_if_else_if_else() {
        let mut context = Context::from_str("if true { 1 } else if false { 2 } else { 3 }");
        let root = Span::from_str("if true { 1 } else if false { 2 } else { 3 }");
        // if true { 1 } else if false { 2 } else { 3 }
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 44),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: root.restrict_range(0, 13),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: root.restrict_range(3, 7),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: root.restrict_range(3, 7),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: root.restrict_range(8, 13),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: root.restrict_range(10, 12),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: root.restrict_range(10, 11),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: root.restrict_range(10, 11),
                                            value: Literal::Int(1),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                },
                else_ifs: vec![Node {
                    id: 8,
                    span: root.restrict_range(14, 33),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 9,
                            span: root.restrict_range(22, 27),
                            value: Expr::Literal(Node {
                                id: 10,
                                span: root.restrict_range(22, 27),
                                value: Literal::Bool(false),
                            }),
                        }),
                        value: Node {
                            id: 11,
                            span: root.restrict_range(28, 33),
                            value: Block {
                                stmts: vec![Node {
                                    id: 12,
                                    span: root.restrict_range(30, 32),
                                    value: Stmt::Expr(Node {
                                        id: 13,
                                        span: root.restrict_range(30, 31),
                                        value: Expr::Literal(Node {
                                            id: 14,
                                            span: root.restrict_range(30, 31),
                                            value: Literal::Int(2),
                                        }),
                                    }),
                                }],
                            },
                        },
                    },
                }],
                else_stmt: Some(Node {
                    id: 15,
                    span: root.restrict_range(39, 44),
                    value: Block {
                        stmts: vec![Node {
                            id: 16,
                            span: root.restrict_range(41, 43),
                            value: Stmt::Expr(Node {
                                id: 17,
                                span: root.restrict_range(41, 42),
                                value: Expr::Literal(Node {
                                    id: 18,
                                    span: root.restrict_range(41, 42),
                                    value: Literal::Int(3),
                                }),
                            }),
                        }],
                    },
                }),
            },
        };
        let actual = Conditional::parse(
            FluxParser::parse(
                Rule::conditional_stmt,
                "if true { 1 } else if false { 2 } else { 3 }",
            )
            .unwrap()
            .next()
            .unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }
}
