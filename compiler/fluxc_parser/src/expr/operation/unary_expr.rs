use fluxc_ast::{Expr, Ident, Literal, UnaryExpr, UnaryOp};
use pest::iterators::Pair;

use crate::{unexpected_rule, Context, PResult, Parse, Rule};

impl Parse for UnaryExpr {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::unary_expr);
        let outer_span = input.as_span();
        let node = ctx.new_empty(input.as_span());
        let mut inner = input.into_inner();
        // peek for prefix operator
        let prefix = match inner.peek() {
            Some(pair) => {
                // if it's a prefix operator, consume it
                if pair.as_rule() == Rule::unary_op_prefix {
                    let inner = pair.into_inner().next().unwrap();
                    Some(match inner.as_rule() {
                        Rule::unary_op_prefix_logical_not => UnaryOp::LogicalNot,
                        Rule::unary_op_prefix_bitwise_not => UnaryOp::BitwiseNot,
                        Rule::unary_op_prefix_reference => UnaryOp::Reference,
                        Rule::unary_op_prefix_dereference => UnaryOp::Dereference,
                        _ => unreachable!(),
                    })
                } else {
                    None
                }
            }
            None => None,
        };
        // get expression
        let expr = inner.next().unwrap();
        let span = expr.as_span();
        let expr = match expr.as_rule() {
            Rule::literal => {
                let value = Expr::Literal(Literal::parse(expr, ctx)?);
                ctx.new_node(span, value)
            }
            Rule::ident => {
                let value = Expr::Ident(Ident::parse(expr, ctx)?);
                ctx.new_node(span, value)
            }
            Rule::expr => Expr::parse(inner.next().unwrap(), ctx)?,
            rule => unexpected_rule(rule, Rule::unary_expr),
        };
        // get postfix operator
        let postfix = match inner.next() {
            Some(pair) => {
                // if it's a postfix operator, consume it
                let inner = pair.into_inner().next().unwrap();
                Some(match inner.as_rule() {
                    Rule::unary_op_prefix_logical_not => UnaryOp::LogicalNot,
                    Rule::unary_op_prefix_bitwise_not => UnaryOp::BitwiseNot,
                    Rule::unary_op_prefix_reference => UnaryOp::Reference,
                    Rule::unary_op_prefix_dereference => UnaryOp::Dereference,
                    Rule::unary_op_postfix_increment => UnaryOp::Increment,
                    Rule::unary_op_postfix_decrement => UnaryOp::Decrement,
                    rule => unexpected_rule(rule, Rule::unary_expr),
                })
            }
            None => None,
        };

        Ok(node.fill(UnaryExpr {
            kind: match &prefix {
                Some(kind) => (*kind).clone(),
                None => postfix.clone().unwrap(),
            },
            expr: match &prefix {
                Some(_) => {
                    let unary_expr_node = ctx.new_node(
                        &outer_span,
                        UnaryExpr { kind: postfix.unwrap(), expr: Box::new(expr) },
                    );
                    Box::new(ctx.new_node(&outer_span, Expr::UnaryExpr(unary_expr_node)))
                }
                None => Box::new(expr),
            },
        }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Expr, Node, UnaryExpr, UnaryOp};
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_unary_expr() {
        let mut ctx = Context::from_str("x--");
        let root = ctx.create_span();
        // x--
        let expected = Node {
            id: 0,
            span: root.restrict_range(0, 2),
            value: UnaryExpr {
                kind: UnaryOp::Decrement,
                expr: Box::new(Node {
                    id: 2,
                    span: root.restrict_range(0, 0),
                    value: Expr::Ident(Node {
                        id: 1,
                        span: root.restrict_range(0, 0),
                        value: "x".to_string(),
                    }),
                }),
            },
        };
        let actual = UnaryExpr::parse(
            FluxParser::parse(Rule::unary_expr, "x--").unwrap().next().unwrap(),
            &mut ctx,
        )
        .unwrap();
        assert_eq!(expected, actual);
    }
}
