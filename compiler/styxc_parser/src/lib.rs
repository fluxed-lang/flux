extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;
extern crate log;

use std::error::Error;

use lazy_static::lazy_static;
use log::debug;
use pest::{
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use styxc_ast::{
    Assignment, BinOp, BinOpKind, Block, Declaration, Expr, Ident, Literal, LiteralKind, Loop,
    Mutability, Span, Stmt, StmtKind, AST,
};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct StyxParser {
    next_id: usize,
}

lazy_static! {
    /// The PrecClimber for parsing binary expressions. Since binary expressions are recursive, and the precedence
    /// of operators cannot easily be inferred, we use the PrecClimber to ensure that the parser grammar will not left recurse.
    /// This has the added benefit of handling operator precedence and associativity properly.
    static ref BIN_EXP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        Operator::new(Rule::bin_op_plus, Assoc::Left)
            | Operator::new(Rule::bin_op_minus, Assoc::Left),
        Operator::new(Rule::bin_op_mul, Assoc::Right)
            | Operator::new(Rule::bin_op_div, Assoc::Right)
    ]);
}

impl StyxParser {
    /// Build the AST by parsing the source.
    pub fn build(&mut self, source: &String) -> Result<AST, Box<dyn Error>> {
        debug!("Building AST from source (len {})", source.len());
        let mut root = Self::parse(Rule::styx, source)?;
        // know that the first rule will be a `statements` rule.
        let stmts = root.next().unwrap().into_inner();
        let stmts = self.parse_statements(stmts);
        debug!("Produced {} top-level AST statements", stmts.len());
        Ok(AST {
            modules: vec![],
            stmts,
        })
    }

    /// Fetch the next AST ID, incrementing the stored `next_id` field.
    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    /// Parse a statements rule into an array of statement AST nodes.    
    fn parse_statements(&mut self, pair: Pairs<Rule>) -> Vec<Stmt> {
        let mut stmts = vec![];
        for inner in pair {
            let stmt = Stmt {
                id: self.next_id(),
                kind: match inner.as_rule() {
                    Rule::declaration => StmtKind::Declaration(self.parse_declaration(inner)),
                    Rule::assignment => StmtKind::Assignment(self.parse_assignment(inner)),
                    Rule::loop_block => StmtKind::Loop(self.parse_loop_block(inner)),
                    Rule::EOI => break,
                    _ => {
                        unreachable!("unexpected match: {:?}", inner.as_rule())
                    }
                },
            };
            stmts.push(stmt);
        }
        stmts
    }

    /// Parse a declaration.
    fn parse_declaration(&mut self, pair: Pair<Rule>) -> Declaration {
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap();
        let value = inner.next().unwrap();
        Declaration {
            ident: self.parse_identifier(ident).into(),
            mutability: Mutability::Immutable,
            value: self.parse_expression(value),
        }
    }

    /// Parse an assignment.
    fn parse_assignment(&mut self, pair: Pair<Rule>) -> Assignment {
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap();
        // =
        inner.next();
        let value = inner.next().unwrap();

        Assignment {
            ident: self.parse_identifier(ident),
            value: self.parse_expression(value),
        }
    }

    /// Parse an identifier.
    fn parse_identifier(&mut self, pair: Pair<Rule>) -> Ident {
        Ident {
            id: 0,
            name: pair.as_str().into(),
            span: Span(pair.as_span().start(), pair.as_span().end()),
        }
    }

    /// Parse an expression.
    fn parse_expression(&mut self, pair: Pair<Rule>) -> Expr {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::ident => Expr::Ident(self.parse_identifier(inner)),
            Rule::literal => Expr::Literal(self.parse_literal(inner)),
            Rule::bin_exp => self.parse_bin_exp(inner),
            _ => unreachable!(),
        }
    }

    /// Parse a literal.
    fn parse_literal(&mut self, pair: Pair<Rule>) -> Literal {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::int => self.parse_int_literal(inner),
            _ => unreachable!(),
        }
    }

    /// Parse an integer literal.
    fn parse_int_literal(&mut self, pair: Pair<Rule>) -> Literal {
        let inner = pair.into_inner().next().unwrap();
        let kind = match inner.as_rule() {
            Rule::num_dec => LiteralKind::Int(inner.as_str().parse().unwrap()),
            Rule::num_hex => LiteralKind::Int(inner.as_str().parse().unwrap()),
            Rule::num_oct => LiteralKind::Int(inner.as_str().parse().unwrap()),
            Rule::num_bin => LiteralKind::Int(inner.as_str().parse().unwrap()),
            _ => unreachable!(),
        };

        Literal {
            id: self.next_id(),
            kind,
            span: Span(inner.as_span().start(), inner.as_span().end()),
        }
    }

    /// Parse a binary expression.
    fn parse_bin_exp(&mut self, pair: Pair<Rule>) -> Expr {
        let inner = pair.into_inner();
        let primary = |pair: Pair<Rule>| match pair.as_rule() {
            Rule::ident => Expr::Ident(self.parse_identifier(pair)),
            Rule::literal => Expr::Literal(self.parse_literal(pair)),
            Rule::expression => self.parse_expression(pair),
            _ => unreachable!(),
        };
        let infix = |lhs: Expr, op: Pair<Rule>, rhs: Expr| {
            Expr::BinOp(BinOp {
                id: 0,
                kind: match op.as_rule() {
                    Rule::bin_op_plus => BinOpKind::Add,
                    Rule::bin_op_minus => BinOpKind::Sub,
                    Rule::bin_op_mul => BinOpKind::Mul,
                    Rule::bin_op_div => BinOpKind::Div,
                    _ => unreachable!(),
                },
                lhs: lhs.into(),
                rhs: rhs.into(),
            })
        };
        BIN_EXP_CLIMBER.climb(inner, primary, infix)
    }

    /// Parse a `loop {}` block.
    fn parse_loop_block(&mut self, pair: Pair<Rule>) -> Loop {
        Loop {
            id: self.next_id(),
            block: self.parse_block(pair.into_inner().next().unwrap()),
        }
    }

    /// Parse a `{ /* ... */}`.
    fn parse_block(&mut self, pair: Pair<Rule>) -> Block {
        debug_assert!(pair.as_rule() == Rule::block);
        let inner = pair.into_inner().next().unwrap().into_inner();
        let stmts = self.parse_statements(inner);
        Block {
            id: self.next_id(),
            stmts,
        }
    }
}

impl Default for StyxParser {
    fn default() -> Self {
        Self { next_id: 0 }
    }
}

#[cfg(test)]
mod tests {
    use pest::Span;
    use styxc_ast::*;

    use super::*;

    #[test]
    fn test_parse() {
        let source: String = "let x = 1; x = z;".into();
        let ast = StyxParser::default().build(&source).unwrap();

        assert_eq!(
            ast,
            AST {
                modules: vec![],
                stmts: vec![
                    Stmt {
                        id: 1,
                        kind: StmtKind::Declaration(Declaration {
                            ident: Ident {
                                id: 0,
                                name: "x".into(),
                                span: Span(4, 5),
                            },
                            mutability: Mutability::Immutable,
                            value: Expr::Literal(Literal {
                                id: 2,
                                kind: LiteralKind::Int(1),
                                span: Span(8, 9),
                            })
                        })
                    },
                    Stmt {
                        id: 3,
                        kind: StmtKind::Assignment(Assignment {
                            ident: Ident {
                                id: 0,
                                name: "x".into(),
                                span: Span(11, 12),
                            },
                            value: Expr::Ident(Ident {
                                id: 0,
                                name: "z".into(),
                                span: Span(15, 16),
                            })
                        })
                    }
                ]
            }
        )
    }

    #[test]
    fn test_bin_op() {
        let source: String = "let x = 1 + 2".into();
        let ast = StyxParser::default().build(&source).unwrap();
        assert_eq!(
            ast,
            AST {
                modules: vec![],
                stmts: vec![Stmt {
                    id: 1,
                    kind: StmtKind::Declaration(Declaration {
                        ident: Ident {
                            id: 0,
                            name: "x".into(),
                            span: Span(4, 5),
                        },
                        mutability: Mutability::Immutable,
                        value: Expr::BinOp(BinOp {
                            id: 0,
                            kind: BinOpKind::Add,
                            lhs: Expr::Literal(Literal {
                                id: 2,
                                kind: LiteralKind::Int(1),
                                span: Span(8, 9),
                            })
                            .into(),
                            rhs: Expr::Literal(Literal {
                                id: 3,
                                kind: LiteralKind::Int(2),
                                span: Span(12, 13),
                            })
                            .into(),
                        })
                    })
                }]
            }
        )
    }

    #[test]
    fn test_bin_op_the_second() {
        let source: String = "let x = 1 + 2 * 3 / 4".into();
        let ast = StyxParser::default().build(&source).unwrap();
        assert_eq!(
            ast,
            AST {
                modules: vec![],
                stmts: vec![Stmt {
                    id: 1,
                    kind: StmtKind::Declaration(Declaration {
                        ident: Ident {
                            id: 0,
                            name: "x".into(),
                            span: Span(4, 5),
                        },
                        mutability: Mutability::Immutable,
                        value: Expr::BinOp(BinOp {
                            id: 0,
                            kind: BinOpKind::Add,
                            lhs: Expr::Literal(Literal {
                                id: 2,
                                kind: LiteralKind::Int(1),
                                span: Span(8, 9),
                            })
                            .into(),
                            rhs: Expr::BinOp(BinOp {
                                id: 0,
                                kind: BinOpKind::Mul,
                                lhs: Expr::Literal(Literal {
                                    id: 3,
                                    kind: LiteralKind::Int(2),
                                    span: Span(12, 13),
                                })
                                .into(),
                                rhs: Expr::BinOp(BinOp {
                                    id: 0,
                                    kind: BinOpKind::Div,
                                    lhs: Expr::Literal(Literal {
                                        id: 4,
                                        kind: LiteralKind::Int(3),
                                        span: Span(16, 17),
                                    })
                                    .into(),
                                    rhs: Expr::Literal(Literal {
                                        id: 5,
                                        kind: LiteralKind::Int(4),
                                        span: Span(20, 21),
                                    })
                                    .into(),
                                })
                                .into()
                            })
                            .into()
                        })
                    })
                }]
            }
        )
    }

    #[test]
    fn test_ident() {
        // x
        let mut res = StyxParser::parse(Rule::ident, "x").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `ident`");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("x", 0, 1).unwrap());
        assert_eq!(res.as_str(), "x");

        // someFunc_1234
        let mut res =
            StyxParser::parse(Rule::ident, "someFunc_1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `ident`");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("someFunc_1234", 0, 13).unwrap());
        assert_eq!(res.as_str(), "someFunc_1234");
    }

    #[test]
    fn test_int() {
        // 1234
        let mut res = StyxParser::parse(Rule::int, "1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("1234", 0, 4).unwrap());
        assert_eq!(res.as_str(), "1234");

        // -4321
        let mut res = StyxParser::parse(Rule::int, "-4321").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-4321", 0, 5).unwrap());
        assert_eq!(res.as_str(), "-4321");

        // 0b1011101
        let mut res = StyxParser::parse(Rule::int, "0b1011101").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0b1011101", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0b1011101");

        // -0d123456890
        let mut res =
            StyxParser::parse(Rule::int, "-0d123456890").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-0d123456890", 0, 12).unwrap());
        assert_eq!(res.as_str(), "-0d123456890");

        // 0o1234567
        let mut res = StyxParser::parse(Rule::int, "0o1234567").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0o1234567", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0o1234567");

        // 0xffff
        let mut res = StyxParser::parse(Rule::int, "0xffff").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0xffff", 0, 6).unwrap());
        assert_eq!(res.as_str(), "0xffff");
    }

    #[test]
    fn test_float() {
        // 1234.5
        let mut res = StyxParser::parse(Rule::float, "1234.5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("1234.5", 0, 6).unwrap());
        assert_eq!(res.as_str(), "1234.5");

        // -543.21
        let mut res = StyxParser::parse(Rule::float, "-543.21").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("-543.21", 0, 7).unwrap());
        assert_eq!(res.as_str(), "-543.21");

        // 23e7
        let mut res = StyxParser::parse(Rule::float, "23e7").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("23e7", 0, 4).unwrap());
        assert_eq!(res.as_str(), "23e7");

        // 32e-72
        let mut res = StyxParser::parse(Rule::float, "32e-72").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("32e-72", 0, 6).unwrap());
        assert_eq!(res.as_str(), "32e-72");
    }

    #[test]
    fn test_char() {
        // 'a'
        let mut res = StyxParser::parse(Rule::char, "'a'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'a'", 0, 3).unwrap());
        assert_eq!(res.as_str(), "'a'");

        // '\n'
        let mut res = StyxParser::parse(Rule::char, "'\\n'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\n'", 0, 4).unwrap());
        assert_eq!(res.as_str(), "'\\n'");

        // '\uFF0F'
        let mut res =
            StyxParser::parse(Rule::char, "'\\uFF0F'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\uFF0F'", 0, 8).unwrap());
        assert_eq!(res.as_str(), "'\\uFF0F'");
    }

    #[test]
    fn test_string() {
        // "hello world"
        let mut res =
            StyxParser::parse(Rule::string, "\"hello world\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello world\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello world\"");

        // "hello, \u60ff"
        let mut res = StyxParser::parse(Rule::string, "\"hello, \\u60ff\"")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(
            res.as_span(),
            Span::new("\"hello, \\u60ff\"", 0, 15).unwrap()
        );
        assert_eq!(res.as_str(), "\"hello, \\u60ff\"");

        // hello, 🦊
        let mut res =
            StyxParser::parse(Rule::string, "\"hello, 🦊\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello, 🦊\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello, 🦊\"");
    }

    #[test]
    fn test_statement() {
        // let x = 5;
        let mut res =
            StyxParser::parse(Rule::statement, "let x = 5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule statement");
        assert_eq!(res.as_rule(), Rule::declaration);
        assert_eq!(res.as_span(), Span::new("let x = 5", 0, 9).unwrap());
        assert_eq!(res.as_str(), "let x = 5");
    }

    #[test]
    fn test_inline_statements() {
        // let x = 5; x = 2\n
        let mut res =
            StyxParser::parse(Rule::line, "let x = 5; x = 2\n").unwrap_or_else(|e| panic!("{}", e));
        let mut stmts = res.next().unwrap().into_inner();
        // let x = 5;
        let stmt = stmts.next().expect("expected match for rule statement");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(
            stmt.as_span(),
            Span::new("let x = 5; x = 2\n", 0, 9).unwrap()
        );
        assert_eq!(stmt.as_str(), "let x = 5");
        // x = 2
        let stmt = stmts.next().expect("expected match for rule statement");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(
            stmt.as_span(),
            Span::new("let x = 5; x = 2\n", 11, 16).unwrap()
        );
        assert_eq!(stmt.as_str(), "x = 2");
    }

    #[test]
    fn test_block() {
        // { let x = 5; x = 2 }
        let mut res = StyxParser::parse(Rule::block, "{ let x = 5; x = 2 }")
            .unwrap_or_else(|e| panic!("{}", e));
        // unwrap block
        let stmt = res.next().expect("expecte match for rule `block`");
        assert_eq!(stmt.as_rule(), Rule::block);
        assert_eq!(
            stmt.as_span(),
            Span::new("{ let x = 5; x = 2 }", 0, 20).unwrap()
        );
        assert_eq!(stmt.as_str(), "{ let x = 5; x = 2 }");
        // unwrap block contents
        let mut inner = stmt.into_inner().next().unwrap().into_inner();
        // let x = 5;
        let stmt = inner.next().expect("expected match for rule `declaration`");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(
            stmt.as_span(),
            Span::new("{ let x = 5; x = 2 }", 2, 11).unwrap()
        );
        assert_eq!(stmt.as_str(), "let x = 5");
        // x = 2
        let stmt = inner.next().expect("expected match for rule `assignment`");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(
            stmt.as_span(),
            Span::new("{ let x = 5; x = 2 }", 13, 18).unwrap()
        );
        assert_eq!(stmt.as_str(), "x = 2");
    }

    #[test]
    fn test_multiline_statements() {
        // let x = 5
        // x = 2
        let mut res =
            StyxParser::parse(Rule::stmts, "let x = 5\nx = 2").unwrap_or_else(|e| panic!("{}", e));
        let mut stmts = res.next().unwrap().into_inner();
        // let x = 5
        let stmt = stmts.next().expect("expected match for rule `declaration`");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(stmt.as_span(), Span::new("let x = 5\nx = 2", 0, 9).unwrap());
        assert_eq!(stmt.as_str(), "let x = 5");
        // x = 2
        let stmt = stmts.next().expect("expected match for rule `assignment`");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(
            stmt.as_span(),
            Span::new("let x = 5\nx = 2", 10, 15).unwrap()
        );
        assert_eq!(stmt.as_str(), "x = 2");
    }

    #[test]
    fn test_multiline_xtreme() {
        // let x = 1; x = 2;
        // let y = 3; y = 4
        // let z = 5;
        // z = 6
        let source = "let x = 1; x = 2\nlet y = 3; y = 4\nlet z = 5\nz = 6";
        let mut res = StyxParser::parse(Rule::stmts, source).unwrap_or_else(|e| panic!("{}", e));
        let mut stmts = res.next().unwrap().into_inner();

        // let x = 1
        let stmt = stmts.next().expect("expected match for rule `declaration`");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(stmt.as_span(), Span::new(source, 0, 9).unwrap());
        assert_eq!(stmt.as_str(), "let x = 1");
        // x = 2
        let stmt = stmts.next().expect("expected match for rule `assignment`");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(stmt.as_span(), Span::new(source, 11, 16).unwrap());
        assert_eq!(stmt.as_str(), "x = 2");
        // let y = 3
        let stmt = stmts.next().expect("expected match for rule `declaration`");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(stmt.as_span(), Span::new(source, 17, 26).unwrap());
        assert_eq!(stmt.as_str(), "let y = 3");
        // y = 4
        let stmt = stmts.next().expect("expected match for rule `assignment`");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(stmt.as_span(), Span::new(source, 28, 33).unwrap());
        assert_eq!(stmt.as_str(), "y = 4");
        // let z = 5
        let stmt = stmts.next().expect("expected match for rule `declaration`");
        assert_eq!(stmt.as_rule(), Rule::declaration);
        assert_eq!(stmt.as_span(), Span::new(source, 34, 43).unwrap());
        assert_eq!(stmt.as_str(), "let z = 5");
        // z = 6
        let stmt = stmts.next().expect("expected match for rule `assignment`");
        assert_eq!(stmt.as_rule(), Rule::assignment);
        assert_eq!(stmt.as_span(), Span::new(source, 44, 49).unwrap());
        assert_eq!(stmt.as_str(), "z = 6");
    }
}
