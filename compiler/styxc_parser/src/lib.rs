extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate lazy_static;
extern crate log;

use std::error::Error;

use lazy_static::lazy_static;
use log::{debug, trace};
use pest::{
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};
use styxc_ast::{
    control::Loop,
    func::{ExternFunc, FuncCall, ParenArgument},
    operations::{Assignment, AssignmentKind, BinaryExpr, BinaryOp},
    Block, Declaration, Expr, Ident, Literal, Node, Stmt, AST, LetDeclaration,
};
use styxc_types::Type;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct StyxParser {
    next_id: usize,
}

lazy_static! {
    /// The precedence climber for parsing binary expressions. Since binary expressions are recursive, and the precedence
    /// of operators cannot easily be inferred, we use the PrecClimber to ensure that the parser grammar will not left recurse.
    /// This has the added benefit of handling operator precedence and associativity properly.
    static ref BIN_EXP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        // 15
        Operator::new(Rule::token_binary_op_assign, Assoc::Left) |
        Operator::new(Rule::token_binary_op_mul_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_div_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_mod_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_plus_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_minus_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_lshift_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_rshift_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_bitwise_and_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_bitwise_or_eq, Assoc::Left) |
        Operator::new(Rule::token_binary_op_bitwise_xor_eq, Assoc::Left),
        // 14
        Operator::new(Rule::token_binary_op_logical_or, Assoc::Right),
        // 13
        Operator::new(Rule::token_binary_op_logical_and, Assoc::Right),
        // 12
        Operator::new(Rule::token_binary_op_eq, Assoc::Right) |
            Operator::new(Rule::token_binary_op_ne, Assoc::Right),
        // 11
        Operator::new(Rule::token_binary_op_lt, Assoc::Right) |
            Operator::new(Rule::token_binary_op_gt, Assoc::Right) |
            Operator::new(Rule::token_binary_op_le, Assoc::Right) |
            Operator::new(Rule::token_binary_op_ge, Assoc::Right),
        // 10
        Operator::new(Rule::token_binary_op_bitwise_or, Assoc::Right),
        // 9
        Operator::new(Rule::token_binary_op_bitwise_xor, Assoc::Right),
        // 8
        Operator::new(Rule::token_binary_op_bitwise_and, Assoc::Right),
        // 7
        Operator::new(Rule::token_binary_op_lshift, Assoc::Right) |
            Operator::new(Rule::token_binary_op_rshift, Assoc::Right),
        // 6
        Operator::new(Rule::token_binary_op_plus, Assoc::Right)
            | Operator::new(Rule::token_binary_op_minus, Assoc::Right),
        // 5
        Operator::new(Rule::token_binary_op_mul, Assoc::Right)
            | Operator::new(Rule::token_binary_op_div, Assoc::Right)
            | Operator::new(Rule::token_binary_op_mod, Assoc::Right)
    ]);
}

impl StyxParser {
    pub fn build<Source: AsRef<str>>(source: Source) -> Result<AST, Vec<Box<dyn Error>>> {
        let src = source.as_ref();
        // Extract a list of Rule::statement from the styx rule.
        let pairs = Self::parse(Rule::styx, src)
            .map(|mut pairs| pairs.next().unwrap().into_inner())
            .map_err(|e| vec![e.into()])?;
        // Attempt to parse statements.
        let mut stmts: Vec<Result<Node<Stmt>, Vec<Box<dyn Error>>>> = vec![];
        for pair in pairs {
            let stmt = Self::parse_statement(pair);
            stmts.push(stmt);
        }
        // Locate any errors in the parsing.
        let contains_errors = stmts.iter().any(|res| res.is_err());
        match contains_errors {
            // Throw errors if there were any. This flattens all errors produced by the parser into a single vector.
            true => Err(stmts
                .into_iter()
                .filter_map(|res| res.err())
                .flat_map(|errs| errs)
                .collect()),
            // Otherwise, build the AST.
            false => Ok(AST {
                stmts: stmts.into_iter().map(|res| res.unwrap()).collect(),
            }),
        }
    }

    fn parse_statement(pair: Pair<Rule>) -> Result<Node<Stmt>, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::statement));

		trace!("{:#?}", pair);

        let span = pair.as_span();
        let inner = pair.into_inner().next().unwrap();
        let stmt = match inner.as_rule() {
            Rule::stmt_mut_decl => Self::parse_stmt_mut_decl(inner)?,
			Rule::stmt_let_decl => Self::parse_stmt_let_decl(inner)?,
            Rule::stmt_const_decl => Self::parse_stmt_const_decl(inner)?,
            Rule::stmt_class_decl => Self::parse_stmt_class_decl(inner)?,
            Rule::stmt_export => Self::parse_stmt_export(inner)?,
            Rule::stmt_func_decl => Self::parse_stmt_func_decl(inner)?,
            Rule::stmt_extern_func_decl => Self::parse_stmt_extern_func_decl(inner)?,
            Rule::stmt_defer => Stmt::Defer(Self::parse_expr(inner.into_inner().next().unwrap())?),
            Rule::stmt_return => {
                Stmt::Return(Self::parse_expr(inner.into_inner().next().unwrap())?)
            }
            Rule::stmt_break => todo!(),
            Rule::expr => Stmt::Expr(Self::parse_expr(inner)?),
            _ => unreachable!(),
        };
        Ok(Node::new(0, span.into(), stmt))
    }

    fn parse_stmt_mut_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_mut_decl));

        todo!()
    }

	fn parse_stmt_let_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
		debug_assert!(matches!(pair.as_rule(), Rule::stmt_let_decl));
		let span = pair.as_span();
		let inner_pairs = pair.into_inner();

		let mut mutable = false;
		let ident = match inner_pairs.next().unwrap().as_rule() {
			Rule::token_mut => {
				mutable = true;
				Self::parse_ident(pair.into_inner().next().unwrap())?
			},
			Rule::ident => {
				Self::parse_ident(pair.into_inner().next().unwrap())?
			}
			_ => unreachable!(),
		};

	}

    fn parse_stmt_const_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_const_decl));
        todo!()
    }

    fn parse_stmt_class_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_class_decl));
        todo!()
    }

    fn parse_stmt_export(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_export));
        todo!()
    }

    fn parse_stmt_func_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_func_decl));
        todo!()
    }

    fn parse_stmt_extern_func_decl(pair: Pair<Rule>) -> Result<Stmt, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::stmt_extern_func_decl));
        todo!()
    }

    fn parse_expr(pair: Pair<Rule>) -> Result<Expr, Vec<Box<dyn Error>>> {
        debug_assert!(matches!(pair.as_rule(), Rule::expr));
        todo!()
    }

	fn parse_ident(pair: Pair<Rule>) -> Result<Ident, Vec<Box<dyn Error>>> {
		debug_assert!(matches!(pair.as_rule(), Rule::ident));
		let span = pair.as_span();
		let ident = Node::new(0, span.into(), pair.as_str().to_string());
		Ok(ident)
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
        let mut res =
            StyxParser::parse(Rule::literal_int, "1234").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("1234", 0, 4).unwrap());
        assert_eq!(res.as_str(), "1234");

        // -4321
        let mut res =
            StyxParser::parse(Rule::literal_int, "-4321").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("-4321", 0, 5).unwrap());
        assert_eq!(res.as_str(), "-4321");

        // 0b1011101
        let mut res =
            StyxParser::parse(Rule::literal_int, "0b1011101").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("0b1011101", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0b1011101");

        // -0d123456890
        let mut res = StyxParser::parse(Rule::literal_int, "-0d123456890")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule int");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("-0d123456890", 0, 12).unwrap());
        assert_eq!(res.as_str(), "-0d123456890");

        // 0o1234567
        let mut res =
            StyxParser::parse(Rule::literal_int, "0o1234567").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("0o1234567", 0, 9).unwrap());
        assert_eq!(res.as_str(), "0o1234567");

        // 0xffff
        let mut res =
            StyxParser::parse(Rule::literal_int, "0xffff").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `int`");
        assert_eq!(res.as_rule(), Rule::literal_int);
        assert_eq!(res.as_span(), Span::new("0xffff", 0, 6).unwrap());
        assert_eq!(res.as_str(), "0xffff");
    }

    #[test]
    fn test_float() {
        // 1234.5
        let mut res =
            StyxParser::parse(Rule::literal_float, "1234.5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::literal_float);
        assert_eq!(res.as_span(), Span::new("1234.5", 0, 6).unwrap());
        assert_eq!(res.as_str(), "1234.5");

        // -543.21
        let mut res =
            StyxParser::parse(Rule::literal_float, "-543.21").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::literal_float);
        assert_eq!(res.as_span(), Span::new("-543.21", 0, 7).unwrap());
        assert_eq!(res.as_str(), "-543.21");

        // 23e7
        let mut res =
            StyxParser::parse(Rule::literal_float, "23e7").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::literal_float);
        assert_eq!(res.as_span(), Span::new("23e7", 0, 4).unwrap());
        assert_eq!(res.as_str(), "23e7");

        // 32e-72
        let mut res =
            StyxParser::parse(Rule::literal_float, "32e-72").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `float`");
        assert_eq!(res.as_rule(), Rule::literal_float);
        assert_eq!(res.as_span(), Span::new("32e-72", 0, 6).unwrap());
        assert_eq!(res.as_str(), "32e-72");
    }

    #[test]
    fn test_char() {
        // 'a'
        let mut res =
            StyxParser::parse(Rule::literal_char, "'a'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::literal_char);
        assert_eq!(res.as_span(), Span::new("'a'", 0, 3).unwrap());
        assert_eq!(res.as_str(), "'a'");

        // '\n'
        let mut res =
            StyxParser::parse(Rule::literal_char, "'\\n'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::literal_char);
        assert_eq!(res.as_span(), Span::new("'\\n'", 0, 4).unwrap());
        assert_eq!(res.as_str(), "'\\n'");

        // '\uFF0F'
        let mut res =
            StyxParser::parse(Rule::literal_char, "'\\uFF0F'").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `char`");
        assert_eq!(res.as_rule(), Rule::literal_char);
        assert_eq!(res.as_span(), Span::new("'\\uFF0F'", 0, 8).unwrap());
        assert_eq!(res.as_str(), "'\\uFF0F'");
    }

    #[test]
    fn test_string() {
        // "hello world"
        let mut res = StyxParser::parse(Rule::literal_string, "\"hello world\"")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::literal_string);
        assert_eq!(res.as_span(), Span::new("\"hello world\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello world\"");

        // "hello, \u60ff"
        let mut res = StyxParser::parse(Rule::literal_string, "\"hello, \\u60ff\"")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::literal_string);
        assert_eq!(
            res.as_span(),
            Span::new("\"hello, \\u60ff\"", 0, 15).unwrap()
        );
        assert_eq!(res.as_str(), "\"hello, \\u60ff\"");

        // hello, 🦊
        let mut res = StyxParser::parse(Rule::literal_string, "\"hello, 🦊\"")
            .unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule `string`");
        assert_eq!(res.as_rule(), Rule::literal_string);
        assert_eq!(res.as_span(), Span::new("\"hello, 🦊\"", 0, 13).unwrap());
        assert_eq!(res.as_str(), "\"hello, 🦊\"");
    }

    #[test]
    fn test_statement() {
        // let x = 5;
        let mut res =
            StyxParser::parse(Rule::statement, "let x = 5").unwrap_or_else(|e| panic!("{}", e));
        let res = res.next().expect("expected match for rule statement");
        let res = res
            .into_inner()
            .next()
            .expect("expected match for rule statement");
        assert_eq!(res.as_rule(), Rule::stmt_let_decl);
        assert_eq!(res.as_span(), Span::new("let x = 5", 0, 9).unwrap());
        assert_eq!(res.as_str(), "let x = 5");
    }
}
