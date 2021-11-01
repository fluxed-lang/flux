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
    control::{If, Loop},
    func::{ExternFunc, FuncCall, ParenArgument},
    operations::{Assignment, AssignmentKind, BinOp, BinOpKind},
    Block, Declaration, Expr, Ident, Literal, LiteralKind, Mutability, Node, Stmt, AST,
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
        Operator::new(Rule::bin_op_log_or, Assoc::Right),
        Operator::new(Rule::bin_op_log_and, Assoc::Right),
        Operator::new(Rule::bin_op_or, Assoc::Right),
        Operator::new(Rule::bin_op_xor, Assoc::Right),
        Operator::new(Rule::bin_op_and, Assoc::Right),
        Operator::new(Rule::bin_op_eq, Assoc::Right) |
            Operator::new(Rule::bin_op_ne, Assoc::Right),
        Operator::new(Rule::bin_op_lt, Assoc::Right) |
            Operator::new(Rule::bin_op_gt, Assoc::Right) |
            Operator::new(Rule::bin_op_le, Assoc::Right) |
            Operator::new(Rule::bin_op_ge, Assoc::Right),
        Operator::new(Rule::bin_op_lshift, Assoc::Right) |
            Operator::new(Rule::bin_op_rshift, Assoc::Right),
        Operator::new(Rule::bin_op_plus, Assoc::Right)
            | Operator::new(Rule::bin_op_minus, Assoc::Right),
        Operator::new(Rule::bin_op_mul, Assoc::Right)
            | Operator::new(Rule::bin_op_div, Assoc::Right)
            | Operator::new(Rule::bin_op_mod, Assoc::Right)
    ]);
}

impl StyxParser {
    /// Build the AST by parsing the source.
    pub fn build(&mut self, source: &String) -> Result<AST, Box<dyn Error>> {
        debug!("Building AST from source (len {})", source.len());
        let mut root = Self::parse(Rule::styx, source)?;
        // know that the first rule will be a `statements` rule.
        let stmts = root.next().unwrap().into_inner();
        let mut stmts = self.parse_statements(stmts)?;
        debug!("Walking the tree to find AST node IDs...");
        self.correct_ids(&mut stmts);
        debug!("Produced {} top-level AST statements", stmts.len());
        trace!("{:#?}", stmts);
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
    fn parse_statements(&mut self, pair: Pairs<Rule>) -> Result<Vec<Node<Stmt>>, Box<dyn Error>> {
        let mut nodes = vec![];
        for inner in pair {
            use Stmt::*;

            let node = Node::new(
                0,
                inner.as_span().into(),
                match inner.as_rule() {
                    Rule::declaration => {
                        Declaration(self.parse_declaration(inner, Mutability::Immutable)?)
                    }
                    Rule::mut_declaration => {
                        Declaration(self.parse_declaration(inner, Mutability::Mutable)?)
                    }
                    Rule::assignment => Assignment(self.parse_assignment(inner)?),
                    Rule::loop_block => Loop(self.parse_loop_block(inner)?),
                    Rule::if_block => If(self.parse_if_statement(inner)?),
                    Rule::func_call => FuncCall(self.parse_func_call(inner)?),
                    Rule::extern_func => ExternFunc(self.parse_extern_func(inner)?),
                    Rule::EOI => break,
                    _ => {
                        unreachable!("unexpected match: {:?}", inner.as_rule())
                    }
                },
            );

            trace!("parsed STATEMENT (id: {})", node.id);
            nodes.push(node);
        }
        Ok(nodes)
    }

    /// Parse a declaration.
    ///
    /// The way this method achieves this is incredibly dumb and needs to be fixed at some point - there is far too much
    /// moving and suspicious data wrangling going on.
    fn parse_declaration(
        &mut self,
        pair: Pair<Rule>,
        mutability: Mutability,
    ) -> Result<Vec<Node<Declaration>>, Box<dyn Error>> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        let mut idents: Vec<(Node<Ident>, Option<Node<Ident>>)> = vec![];
        let mut exprs = vec![];
        // concatenate all idents
        loop {
            let next = inner.next().unwrap();
            if matches!(next.as_rule(), Rule::expression) {
                exprs.push(next);
                break;
            }
            let ident = self.parse_identifier(next)?;
            if matches!(
                inner.peek().map(|r| r.as_rule()),
                Some(Rule::declaration_type)
            ) {
                let type_ident = self.parse_type_ident(inner.next().unwrap())?;
                idents.push((ident, Some(type_ident)))
            } else {
                idents.push((ident, None));
            }
        }
        // concatenate all exprs
        while let Some(expr) = inner.next() {
            exprs.push(expr);
        }
        // panic if mismatching number of exprs and idents
        let single_expr = exprs.len() == 1;
        if !single_expr && exprs.len() != idents.len() {
            return Err("mismatching declaration statement".into());
        }
        // iterate over idents and set
        let mut index = 0;

        let mut results = vec![];
        for (ident, ty_ident) in idents {
            let value = if single_expr {
                &exprs[0]
            } else {
                &exprs[index]
            };
            index += 1;
            let value = self.parse_expression(value.clone())?;
            results.push(Node::new(
                0,
                span,
                Declaration {
                    ty: Type::Infer,
                    ty_ident,
                    ident,
                    mutability,
                    value,
                },
            ))
        }
        Ok(results)
    }

    /// Parse an assignment.
    fn parse_assignment(&mut self, pair: Pair<Rule>) -> Result<Node<Assignment>, Box<dyn Error>> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap();
        // =
        let op = inner.next().unwrap().as_str();
        let value = inner.next().unwrap();

        Ok(Node::new(
            0,
            span,
            Assignment {
                ident: self.parse_identifier(ident)?,
                value: self.parse_expression(value)?,
                kind: match op {
                    "=" => AssignmentKind::Assign,
                    "+=" => AssignmentKind::AddAssign,
                    "-=" => AssignmentKind::SubAssign,
                    "*=" => AssignmentKind::MulAssign,
                    "/=" => AssignmentKind::DivAssign,
                    "%=" => AssignmentKind::ModAssign,
                    "&=" => AssignmentKind::AndAssign,
                    "|=" => AssignmentKind::OrAssign,
                    "^=" => AssignmentKind::XorAssign,
                    _ => unreachable!(),
                },
            },
        ))
    }

    /// Parse an identifier.
    fn parse_identifier(&mut self, pair: Pair<Rule>) -> Result<Node<Ident>, Box<dyn Error>> {
        Ok(Node::new(
            0,
            pair.as_span().into(),
            Ident {
                name: pair.as_str().into(),
            },
        ))
    }

    /// Parse an expression.
    fn parse_expression(
        &mut self,
        pair: Pair<Rule>,
    ) -> Result<Node<Expr>, Box<(dyn Error + 'static)>> {
        let inner = pair.into_inner().next().unwrap();
        Ok(match inner.as_rule() {
			Rule::func_call => Node::new(0, inner.as_span().into(), Expr::FuncCall(self.parse_func_call(inner)?)),
            Rule::ident => Node::new(
                0,
                inner.as_span().into(),
                Expr::Ident(self.parse_identifier(inner)?),
            ),
            Rule::literal => Node::new(
                0,
                inner.as_span().into(),
                Expr::Literal(self.parse_literal(inner)?),
            ),
            Rule::bin_exp => self.parse_bin_exp(inner)?,
            _ => unreachable!(),
        })
    }

    /// Parse a literal.
    fn parse_literal(&mut self, pair: Pair<Rule>) -> Result<Node<Literal>, Box<dyn Error>> {
        let inner = pair.into_inner().next().unwrap();
        Ok(match inner.as_rule() {
            Rule::int => self.parse_int_literal(inner)?,
            Rule::string => self.parse_string_literal(inner)?,
			Rule::bool => self.parse_bool_literal(inner)?,
            _ => unreachable!(),
        })
    }

    /// Parse an integer literal.
    fn parse_int_literal(&mut self, pair: Pair<Rule>) -> Result<Node<Literal>, Box<dyn Error>> {
        Ok(Node::new(
            0,
            pair.as_span().into(),
            Literal {
                ty: Type::Infer,
                kind: LiteralKind::Int(pair.as_str().parse()?),
            },
        ))
    }

    /// Parse a string literal.
    fn parse_string_literal(&mut self, pair: Pair<Rule>) -> Result<Node<Literal>, Box<dyn Error>> {
        let inner = pair.clone().into_inner().next().unwrap().as_str();
        Ok(Node::new(
            0,
            pair.as_span().into(),
            Literal {
                ty: Type::Infer,
                kind: LiteralKind::String(inner.to_string()),
            },
        ))
    }

	fn parse_bool_literal(&mut self, pair: Pair<Rule>) -> Result<Node<Literal>, Box<dyn Error>> {
		Ok(Node::new(0, pair.as_span().into(), Literal {
			ty: Type::Infer,
			kind: LiteralKind::Bool(pair.as_str().parse()?),
		}))
	}

    /// Parse a binary expression.
    fn parse_bin_exp(&mut self, pair: Pair<Rule>) -> Result<Node<Expr>, Box<dyn Error>> {
        let span = pair.as_span().into();
        let inner = pair.into_inner();
        let primary = |pair: Pair<Rule>| match pair.as_rule() {
            Rule::ident => Node::new(0, span, Expr::Ident(self.parse_identifier(pair).unwrap())),
            Rule::literal => Node::new(0, span, Expr::Literal(self.parse_literal(pair).unwrap())),
            Rule::expression => self.parse_expression(pair).unwrap(),
            _ => unreachable!(),
        };
        let infix = |lhs: Node<Expr>, op: Pair<Rule>, rhs: Node<Expr>| {
            Node::new(
                0,
                span,
                Expr::BinOp(Node::new(
                    0,
                    span,
                    BinOp {
                        kind: match op.as_rule() {
                            Rule::bin_op_plus => BinOpKind::Add,
                            Rule::bin_op_minus => BinOpKind::Sub,
                            Rule::bin_op_mul => BinOpKind::Mul,
                            Rule::bin_op_div => BinOpKind::Div,
                            Rule::bin_op_mod => BinOpKind::Mod,
                            Rule::bin_op_and => BinOpKind::And,
                            Rule::bin_op_or => BinOpKind::Or,
                            Rule::bin_op_xor => BinOpKind::Xor,
                            Rule::bin_op_eq => BinOpKind::Eq,
                            Rule::bin_op_ne => BinOpKind::Ne,
                            Rule::bin_op_lt => BinOpKind::Lt,
                            Rule::bin_op_le => BinOpKind::Le,
                            Rule::bin_op_gt => BinOpKind::Gt,
                            Rule::bin_op_ge => BinOpKind::Ge,
                            Rule::bin_op_log_and => BinOpKind::LogAnd,
                            Rule::bin_op_log_or => BinOpKind::LogOr,
                            _ => unreachable!(),
                        },
                        lhs: lhs.into(),
                        rhs: rhs.into(),
                    },
                )),
            )
        };
        Ok(BIN_EXP_CLIMBER.climb(inner, primary, infix))
    }

    /// Parse a `loop {}` block.
    fn parse_loop_block(&mut self, pair: Pair<Rule>) -> Result<Node<Loop>, Box<dyn Error>> {
        Ok(Node::new(
            0,
            pair.as_span().into(),
            Loop {
                block: self.parse_block(pair.into_inner().next().unwrap())?,
            },
        ))
    }

    /// Parse a `{ /* ... */}`.
    fn parse_block(&mut self, pair: Pair<Rule>) -> Result<Node<Block>, Box<dyn Error>> {
        debug_assert!(pair.as_rule() == Rule::block);
        let span = pair.as_span().into();
        let stmts = match pair.into_inner().next() {
            Some(stmts) => self.parse_statements(stmts.into_inner())?,
            None => vec![],
        };
        Ok(Node::new(0, span, Block { stmts }))
    }

    /// Parse a `if {}` block.
    fn parse_if_statement(&mut self, pair: Pair<Rule>) -> Result<Node<If>, Box<dyn Error>> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        let expr = self.parse_expression(inner.next().unwrap())?;
        let block = self.parse_block(inner.next().unwrap())?;
        Ok(Node::new(0, span, If { expr, block }))
    }

    /// Parse an external function declaration statement.
    fn parse_extern_func(&mut self, pair: Pair<Rule>) -> Result<Node<ExternFunc>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut inner = pair.into_inner();
        let ident = self.parse_identifier(inner.next().unwrap())?;
        let args = self.parse_paren_arguments(inner.next().unwrap())?;
        // get the return type of the function if there is one
        let ret_ty_ident = match inner.next() {
            Some(type_ident) => Some(self.parse_type_ident(type_ident)?),
            None => None,
        };
        Ok(Node::new(
            0,
            span.into(),
            ExternFunc {
                ident,
                args,
                ret_ty_ident,
                ty: Type::Infer,
            },
        ))
    }

    /// Parse function parameters.
    fn parse_paren_arguments(
        &mut self,
        pair: Pair<Rule>,
    ) -> Result<Vec<Node<ParenArgument>>, Box<dyn Error>> {
        let mut inner = pair.into_inner();
        let mut params = vec![];
        while let Some(param) = inner.next() {
            params.push(self.parse_paren_argument(param)?);
        }
        Ok(params)
    }

    /// Parse a function parameter.
    fn parse_paren_argument(
        &mut self,
        pair: Pair<Rule>,
    ) -> Result<Node<ParenArgument>, Box<dyn Error>> {
        let span = pair.as_span().into();
        let mut inner = pair.into_inner();
        let ident = self.parse_identifier(inner.next().unwrap())?;
        let ty_ident = self.parse_type_ident(inner.next().unwrap())?;
        Ok(Node::new(
            0,
            span,
            ParenArgument {
                ty: Type::Infer,
                ident,
                ty_ident,
            },
        ))
    }

    /// Parse a type identifier.
    fn parse_type_ident(&mut self, pair: Pair<Rule>) -> Result<Node<Ident>, Box<dyn Error>> {
        let inner = pair.into_inner().next().unwrap();
        if let Rule::builtin_type = inner.as_rule() {
            Ok(Node::new(
                0,
                inner.as_span().into(),
                Ident {
                    name: inner.as_str().to_string(),
                },
            ))
        } else {
            Ok(self.parse_identifier(inner)?)
        }
    }

    /// Parse a function call.
    fn parse_func_call(&mut self, pair: Pair<Rule>) -> Result<Node<FuncCall>, Box<dyn Error>> {
        let span = pair.as_span();
        let mut inner = pair.into_inner();
        let ident = self.parse_identifier(inner.next().unwrap())?;
        let args = self.parse_func_call_params(inner.next().unwrap())?;
        Ok(Node::new(
            0,
            span.into(),
            FuncCall {
                ident,
                args,
                return_ty: Type::Infer,
            },
        ))
    }

    /// Parse function call parameters
    fn parse_func_call_params(
        &mut self,
        pair: Pair<Rule>,
    ) -> Result<Vec<Node<Expr>>, Box<dyn Error>> {
        let mut inner = pair.into_inner();
        let mut params = vec![];
        while let Some(param) = inner.next() {
            params.push(self.parse_expression(param)?);
        }
        Ok(params)
    }

    /// Walk the AST and correct AST node IDs.
    fn correct_ids(&mut self, stmts: &mut Vec<Node<Stmt>>) {
        for stmt in stmts {
            self.correct_stmt_ids(stmt);
        }
    }

    fn correct_stmt_ids(&mut self, stmt: &mut Node<Stmt>) {
        stmt.id = self.next_id();
        match &mut stmt.value {
            Stmt::Declaration(decls) => {
                for decl in decls {
                    decl.id = self.next_id();
                    decl.value.ident.id = self.next_id();
                    if let Some(ty_ident) = &mut decl.value.ty_ident {
                        ty_ident.id = self.next_id();
                    }
                    self.correct_expr_ids(&mut decl.value.value);
                }
            }
            Stmt::Assignment(assign) => {
                assign.id = self.next_id();
                assign.value.ident.id = self.next_id();
                self.correct_expr_ids(&mut assign.value.value);
            }
            Stmt::Loop(loop_block) => {
                loop_block.id = self.next_id();
                loop_block.value.block.id = self.next_id();
                self.correct_ids(&mut loop_block.value.block.value.stmts);
            }
            Stmt::If(if_block) => {
                if_block.id = self.next_id();
                self.correct_expr_ids(&mut if_block.value.expr);
                if_block.value.block.id = self.next_id();
                self.correct_ids(&mut if_block.value.block.value.stmts);
            }
            Stmt::FuncDecl(decl) => {
                decl.id = self.next_id();
                decl.value.ident.id = self.next_id();
                for arg in &mut decl.value.args {
                    arg.id = self.next_id();
                    arg.value.ident.id = self.next_id();
                    arg.value.ty_ident.id = self.next_id();
                }
                decl.value.body.id = self.next_id();
                self.correct_ids(&mut decl.value.body.value.stmts);
            }
            Stmt::ExternFunc(extern_func) => {
                extern_func.id = self.next_id();
                extern_func.value.ident.id = self.next_id();
                for arg in &mut extern_func.value.args {
                    arg.id = self.next_id();
                    arg.value.ident.id = self.next_id();
                    arg.value.ty_ident.id = self.next_id();
                }
                if let Some(ret_ty_ident) = &mut extern_func.value.ret_ty_ident {
                    ret_ty_ident.id = self.next_id();
                }
            }
            Stmt::FuncCall(func_call) => {
                func_call.id = self.next_id();
                func_call.value.ident.id = self.next_id();
                for arg in &mut func_call.value.args {
                    self.correct_expr_ids(arg);
                }
            }
            Stmt::Return(ret) => self.correct_expr_ids(ret),
        }
    }

    fn correct_expr_ids(&mut self, expr: &mut Node<Expr>) {
        expr.id = self.next_id();
        match &mut expr.value {
            Expr::Literal(literal) => {
                literal.id = self.next_id();
            }
            Expr::Ident(ident) => {
                ident.id = self.next_id();
            }
            Expr::BinOp(bin_op) => {
                bin_op.id = self.next_id();
                self.correct_expr_ids(&mut bin_op.value.lhs);
                self.correct_expr_ids(&mut bin_op.value.rhs);
            }
            Expr::Block(block) => {
                block.id = self.next_id();
                self.correct_ids(&mut block.value.stmts);
            }
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
