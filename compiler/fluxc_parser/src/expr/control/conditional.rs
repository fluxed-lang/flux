use fluxc_ast::{Block, Conditional, Expr, IfStmt, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Conditional {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {
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
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {
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
        let mut context = Context::default();
        // if true { 1 }
        let expected = Node {
            id: 0,
            span: Span::new(0, 12),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: Span::new(0, 12),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: Span::new(3, 6),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: Span::new(3, 6),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: Span::new(8, 12),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: Span::new(10, 11),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: Span::new(10, 10),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: Span::new(10, 10),
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
        let mut context = Context::default();
        // if true { 1 } else { 2 }
        let expected = Node {
            id: 0,
            span: Span::new(0, 23),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: Span::new(0, 12),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: Span::new(3, 6),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: Span::new(3, 6),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: Span::new(8, 12),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: Span::new(10, 11),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: Span::new(10, 10),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: Span::new(10, 10),
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
                    span: Span::new(19, 23),
                    value: Block {
                        stmts: vec![Node {
                            id: 9,
                            span: Span::new(21, 22),
                            value: Stmt::Expr(Node {
                                id: 10,
                                span: Span::new(21, 21),
                                value: Expr::Literal(Node {
                                    id: 11,
                                    span: Span::new(21, 21),
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
        let mut context = Context::default();
        // if true { 1 } else if false { 2 }
        let expected = Node {
            id: 0,
            span: Span::new(0, 32),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: Span::new(0, 12),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: Span::new(3, 6),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: Span::new(3, 6),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: Span::new(8, 12),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: Span::new(10, 11),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: Span::new(10, 10),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: Span::new(10, 10),
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
                    span: Span::new(14, 32),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 9,
                            span: Span::new(22, 26),
                            value: Expr::Literal(Node {
                                id: 10,
                                span: Span::new(22, 26),
                                value: Literal::Bool(false),
                            }),
                        }),
                        value: Node {
                            id: 11,
                            span: Span::new(28, 32),
                            value: Block {
                                stmts: vec![Node {
                                    id: 12,
                                    span: Span::new(30, 31),
                                    value: Stmt::Expr(Node {
                                        id: 13,
                                        span: Span::new(30, 30),
                                        value: Expr::Literal(Node {
                                            id: 14,
                                            span: Span::new(30, 30),
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
        let mut context = Context::default();
        // if true { 1 } else if false { 2 } else { 3 }
        let expected = Node {
            id: 0,
            span: Span::new(0, 43),
            value: Conditional {
                if_stmt: Node {
                    id: 1,
                    span: Span::new(0, 12),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 2,
                            span: Span::new(3, 6),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: Span::new(3, 6),
                                value: Literal::Bool(true),
                            }),
                        }),
                        value: Node {
                            id: 4,
                            span: Span::new(8, 12),
                            value: Block {
                                stmts: vec![Node {
                                    id: 5,
                                    span: Span::new(10, 11),
                                    value: Stmt::Expr(Node {
                                        id: 6,
                                        span: Span::new(10, 10),
                                        value: Expr::Literal(Node {
                                            id: 7,
                                            span: Span::new(10, 10),
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
                    span: Span::new(14, 32),
                    value: IfStmt {
                        condition: Box::new(Node {
                            id: 9,
                            span: Span::new(22, 26),
                            value: Expr::Literal(Node {
                                id: 10,
                                span: Span::new(22, 26),
                                value: Literal::Bool(false),
                            }),
                        }),
                        value: Node {
                            id: 11,
                            span: Span::new(28, 32),
                            value: Block {
                                stmts: vec![Node {
                                    id: 12,
                                    span: Span::new(30, 31),
                                    value: Stmt::Expr(Node {
                                        id: 13,
                                        span: Span::new(30, 30),
                                        value: Expr::Literal(Node {
                                            id: 14,
                                            span: Span::new(30, 30),
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
                    span: Span::new(39, 43),
                    value: Block {
                        stmts: vec![Node {
                            id: 16,
                            span: Span::new(41, 42),
                            value: Stmt::Expr(Node {
                                id: 17,
                                span: Span::new(41, 41),
                                value: Expr::Literal(Node {
                                    id: 18,
                                    span: Span::new(41, 41),
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
