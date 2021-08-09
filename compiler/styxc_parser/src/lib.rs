extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::error::Error;

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use styxc_ast::{
    Assignment, Declaration, Expr, Ident, Literal, LiteralKind, Mutability, Span, Stmt, StmtKind,
    AST,
};

/// A trait implemented in this crate for AST elements, allowing them to be parsed.
trait Parse<T> {
    /// Parse the given pair into this AST element.
    fn parse(pair: Pair<Rule>) -> T;
}

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct StyxParser {
    next_id: usize,
}

impl StyxParser {
    /// Build the AST by parsing the source.
    pub fn build(&mut self, source: &String) -> Result<AST, Box<dyn Error>> {
        let mut root = Self::parse(Rule::root, source)?;
        // know that the first rule will be a `statements` rule.
        let stmts = root.next().unwrap().into_inner();
        let stmts = self.parse_statements(stmts);

        Ok(AST {
            modules: vec![],
            stmts,
        })
    }

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
        // access assignment rule
        let mut inner = pair.into_inner().next().unwrap().into_inner();
        let ident = inner.next().unwrap();
        // =
        inner.next();
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
            _ => unreachable!(),
        }
    }

    /// Parse a literal.
    fn parse_literal(&mut self, pair: Pair<Rule>) -> Literal {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::int => Literal {
                id: self.next_id(),
                kind: LiteralKind::Int64(0),
                span: Span(inner.as_span().start(), inner.as_span().end()),
            },
            _ => unreachable!(),
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
                                kind: LiteralKind::Int64(0),
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
    fn test_ident() {
        // x
        let mut res = StyxParser::parse(Rule::ident, "x").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule ident");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("x", 0, 1).unwrap());
        assert_eq!(res.as_str(), "x");

        // someFunc_1234
        let mut res =
            StyxParser::parse(Rule::ident, "someFunc_1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule ident");

        assert_eq!(res.as_rule(), Rule::ident);
        assert_eq!(res.as_span(), Span::new("someFunc_1234", 0, 13).unwrap());
        assert_eq!(res.as_str(), "someFunc_1234");
    }

    #[test]
    fn test_int() {
        // 1234
        let mut res = StyxParser::parse(Rule::int, "1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("1234", 0, 4).unwrap());
        assert_eq!(res.as_str(), "1234");

        // -4321
        let mut res = StyxParser::parse(Rule::int, "-4321").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-4321", 0, 5).unwrap());
        assert_eq!(res.as_str(), "-4321");

        // 0b1011101
        let mut res = StyxParser::parse(Rule::int, "0b1011101").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0b1011101", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0b1011101");

        // -0d123456890
        let mut res =
            StyxParser::parse(Rule::int, "-0d123456890").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("-0d123456890", 0, 12).unwrap());
        assert_eq!(res.as_str(), "-0d123456890");

        // 0o1234567
        let mut res = StyxParser::parse(Rule::int, "0o1234567").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0o1234567", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0o1234567");

        // 0xffff
        let mut res = StyxParser::parse(Rule::int, "0xffff").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule int");
        assert_eq!(res.as_rule(), Rule::int);
        assert_eq!(res.as_span(), Span::new("0xffff", 0, 6).unwrap());
        assert_eq!(res.as_str(), "0xffff");
    }

    #[test]
    fn test_float() {
        // 1234.5
        let mut res = StyxParser::parse(Rule::float, "1234.5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("1234.5", 0, 6).unwrap());
        assert_eq!(res.as_str(), "1234.5");

        // -543.21
        let mut res = StyxParser::parse(Rule::float, "-543.21").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("-543.21", 0, 7).unwrap());
        assert_eq!(res.as_str(), "-543.21");

        // 23e7
        let mut res = StyxParser::parse(Rule::float, "23e7").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("23e7", 0, 4).unwrap());
        assert_eq!(res.as_str(), "23e7");

        // 32e-72
        let mut res = StyxParser::parse(Rule::float, "32e-72").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule float");
        assert_eq!(res.as_rule(), Rule::float);
        assert_eq!(res.as_span(), Span::new("32e-72", 0, 6).unwrap());
        assert_eq!(res.as_str(), "32e-72");
    }

    #[test]
    fn test_char() {
        // 'a'
        let mut res = StyxParser::parse(Rule::char, "'a'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'a'", 0, 3).unwrap());
        assert_eq!(res.as_str(), "'a'");

        // '\n'
        let mut res = StyxParser::parse(Rule::char, "'\\n'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\n'", 0, 4).unwrap());
        assert_eq!(res.as_str(), "'\\n'");

        // '\uFF0F'
        let mut res =
            StyxParser::parse(Rule::char, "'\\uFF0F'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule char");
        assert_eq!(res.as_rule(), Rule::char);
        assert_eq!(res.as_span(), Span::new("'\\uFF0F'", 0, 8).unwrap());
        assert_eq!(res.as_str(), "'\\uFF0F'");
    }

    #[test]
    fn test_string() {
        // "hello world"
        let mut res =
            StyxParser::parse(Rule::string, "\"hello world\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello world\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello world\"");

        // "hello, \u60ff"
        let mut res = StyxParser::parse(Rule::string, "\"hello, \\u60ff\"")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(
            res.as_span(),
            Span::new("\"hello, \\u60ff\"", 0, 15).unwrap()
        );
        assert_eq!(res.as_str(), "\"hello, \\u60ff\"");

        // hello, 🦊
        let mut res =
            StyxParser::parse(Rule::string, "\"hello, 🦊\"").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("Expected match for rule string");
        assert_eq!(res.as_rule(), Rule::string);
        assert_eq!(res.as_span(), Span::new("\"hello, 🦊\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello, 🦊\"");
    }
}
