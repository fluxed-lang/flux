use std::sync::Mutex;

use fluxc_ast::{BinaryExpr, BinaryOp, Expr, Ident, Literal, Node};
use fluxc_errors::CompilerError;

use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
};

use crate::{Context, PResult, Parse, Rule};

lazy_static! {
    /// The precedence climber for parsing binary expressions. Since binary expressions are recursive, and the precedence
    /// of operators cannot easily be inferred, we use the PrecClimber to ensure that the parser grammar will not left recurse.
    /// This has the added benefit of handling operator precedence and associativity properly.
    static ref BIN_EXP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        // 15
        Operator::new(Rule::binary_op_assign, Assoc::Left) |
        Operator::new(Rule::binary_op_mul_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_div_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_mod_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_plus_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_minus_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_lshift_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_rshift_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_and_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_or_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_xor_eq, Assoc::Left),
        // 14
        Operator::new(Rule::binary_op_logical_or, Assoc::Right),
        // 13
        Operator::new(Rule::binary_op_logical_and, Assoc::Right),
        // 12
        Operator::new(Rule::binary_op_eq, Assoc::Right) |
            Operator::new(Rule::binary_op_ne, Assoc::Right),
        // 11
        Operator::new(Rule::binary_op_lt, Assoc::Right) |
            Operator::new(Rule::binary_op_gt, Assoc::Right) |
            Operator::new(Rule::binary_op_le, Assoc::Right) |
            Operator::new(Rule::binary_op_ge, Assoc::Right),
        // 10
        Operator::new(Rule::binary_op_bitwise_or, Assoc::Right),
        // 9
        Operator::new(Rule::binary_op_bitwise_xor, Assoc::Right),
        // 8
        Operator::new(Rule::binary_op_bitwise_and, Assoc::Right),
        // 7
        Operator::new(Rule::binary_op_lshift, Assoc::Right) |
            Operator::new(Rule::binary_op_rshift, Assoc::Right),
        // 6
        Operator::new(Rule::binary_op_plus, Assoc::Right)
            | Operator::new(Rule::binary_op_minus, Assoc::Right),
        // 5
        Operator::new(Rule::binary_op_mul, Assoc::Right)
            | Operator::new(Rule::binary_op_div, Assoc::Right)
            | Operator::new(Rule::binary_op_mod, Assoc::Right)
    ]);
}

impl Parse for BinaryExpr {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::binary_expr);
        let inner = input.into_inner();
        let ctx = Mutex::new(ctx);
        // run the precedence climber
        let out = BIN_EXP_CLIMBER.climb(
            inner,
            |pair: Pair<Rule>| {
                let mut ctx = ctx.lock().unwrap();
                let node = ctx.new_empty(pair.as_span());
                Ok(match pair.as_rule() {
                    Rule::literal => node.fill(Expr::Literal(Literal::parse(pair, &mut ctx)?)),
                    Rule::ident => node.fill(Expr::Ident(Ident::parse(pair, &mut ctx)?)),
                    Rule::expr => Expr::parse(pair, &mut ctx)?,
                    r => unreachable!("{:?}", r),
                })
            },
            |lhs: Result<Node<Expr>, CompilerError>,
             op: Pair<Rule>,
             rhs: Result<Node<Expr>, CompilerError>| {
                let lhs = lhs?;
                let rhs = rhs?;

                let kind = match op.as_rule() {
                    Rule::binary_op_assign => BinaryOp::Assign,
                    Rule::binary_op_bitwise_and => BinaryOp::BitwiseAnd,
                    Rule::binary_op_bitwise_and_eq => BinaryOp::BitwiseAndEq,
                    Rule::binary_op_bitwise_or => BinaryOp::BitwiseOr,
                    Rule::binary_op_bitwise_or_eq => BinaryOp::BitwiseOrEq,
                    Rule::binary_op_bitwise_xor => BinaryOp::BitwiseXor,
                    Rule::binary_op_bitwise_xor_eq => BinaryOp::BitwiseXorEq,
                    Rule::binary_op_div => BinaryOp::Div,
                    Rule::binary_op_div_eq => BinaryOp::DivEq,
                    Rule::binary_op_eq => BinaryOp::Eq,
                    Rule::binary_op_ge => BinaryOp::Ge,
                    Rule::binary_op_gt => BinaryOp::Gt,
                    Rule::binary_op_le => BinaryOp::Le,
                    Rule::binary_op_logical_and => BinaryOp::LogicalAnd,
                    Rule::binary_op_logical_and_eq => BinaryOp::LogicalAndEq,
                    Rule::binary_op_logical_or => BinaryOp::LogicalOr,
                    Rule::binary_op_logical_or_eq => BinaryOp::LogicalOrEq,
                    Rule::binary_op_lshift => BinaryOp::Shl,
                    Rule::binary_op_lshift_eq => BinaryOp::ShlEq,
                    Rule::binary_op_lt => BinaryOp::Lt,
                    Rule::binary_op_minus => BinaryOp::Minus,
                    Rule::binary_op_minus_eq => BinaryOp::MinusEq,
                    Rule::binary_op_mod => BinaryOp::Mod,
                    Rule::binary_op_mod_eq => BinaryOp::ModEq,
                    Rule::binary_op_mul => BinaryOp::Mul,
                    Rule::binary_op_mul_eq => BinaryOp::MulEq,
                    Rule::binary_op_ne => BinaryOp::Ne,
                    Rule::binary_op_plus => BinaryOp::Plus,
                    Rule::binary_op_plus_eq => BinaryOp::PlusEq,
                    Rule::binary_op_rshift => BinaryOp::Shr,
                    Rule::binary_op_rshift_eq => BinaryOp::ShrEq,
                    _ => unreachable!(),
                };
                // acquire lock and create nodes
                let (expr_node, bin_expr_node) = {
                    let mut ctx = ctx.lock().unwrap();
                    let span = ctx.create_span();
                    (
                        ctx.new_empty(span.restrict_range(lhs.span.start, rhs.span.end)),
                        ctx.new_empty(span.restrict_range(lhs.span.start, rhs.span.end)),
                    )
                };
                Ok(expr_node.fill(Expr::BinaryExpr(bin_expr_node.fill(BinaryExpr {
                    kind,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }))))
            },
        );

        match out?.value {
            Expr::BinaryExpr(mut node) => {
                // correct binary expression id
                node.id -= 1;
                ctx.lock().unwrap().next_id -= 1;
                Ok(node)
            }
            r => unreachable!("{:?}", r),
        }
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{BinaryExpr, BinaryOp, Expr, Literal, Node};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{sealed::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_literal_binary_expr() {
        let mut context = Context::from_str("1 * 2 + 3");
        let root = Span::from_str("1 * 2 + 3");
        // 1 * 2 + 3
        let expected = Node {
            id: 8,
            span: root.restrict_range(0, 9),
            value: BinaryExpr {
                kind: BinaryOp::Plus,
                lhs: Box::new(Node {
                    id: 4,
                    span: root.restrict_range(0, 5),
                    value: Expr::BinaryExpr(Node {
                        id: 5,
                        span: root.restrict_range(0, 5),
                        value: BinaryExpr {
                            kind: BinaryOp::Mul,
                            lhs: Box::new(Node {
                                id: 0,
                                span: root.restrict_range(0, 1),
                                value: Expr::Literal(Node {
                                    id: 1,
                                    span: root.restrict_range(0, 1),
                                    value: Literal::Int(1),
                                }),
                            }),
                            rhs: Box::new(Node {
                                id: 2,
                                span: root.restrict_range(4, 5),
                                value: Expr::Literal(Node {
                                    id: 3,
                                    span: root.restrict_range(4, 5),
                                    value: Literal::Int(2),
                                }),
                            }),
                        },
                    }),
                }),
                rhs: Box::new(Node {
                    id: 6,
                    span: root.restrict_range(8, 9),
                    value: Expr::Literal(Node {
                        id: 7,
                        span: root.restrict_range(8, 9),
                        value: Literal::Int(3),
                    }),
                }),
            },
        };
        let result = FluxParser::parse(Rule::binary_expr, "1 * 2 + 3").unwrap().next().unwrap();
        let result = BinaryExpr::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }
}
