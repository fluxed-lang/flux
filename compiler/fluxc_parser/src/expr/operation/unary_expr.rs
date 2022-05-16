use fluxc_ast::{Expr, Node, UnaryExpr, UnaryOp};
use fluxc_errors::CompilerError;
use fluxc_span::Span;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for UnaryExpr {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {
        debug_assert_eq!(input.as_rule(), Rule::unary_expr);
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
        let expr = Expr::parse(inner.next().unwrap(), ctx)?;
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
                    _ => unreachable!(),
                })
            }
            None => None,
        };

        Ok(node.fill(UnaryExpr {
            kind: match &prefix {
                Some(kind) => (*kind).clone(),
                None => postfix.unwrap(),
            },
            expr: match &prefix {
                Some(_) => Box::new(ctx.new_node(
                    Span::new(0, 0),
                    Expr::UnaryExpr(ctx.new_node(
                        Span::new(0, 0),
                        UnaryExpr { kind: postfix.unwrap(), expr: Box::new(expr) },
                    )),
                )),
                None => Box::new(expr),
            },
        }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Expr, Node, UnaryExpr, UnaryOp};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_unary_expr() {
        let mut ctx = Context::default();
        // x--
        let expected = Node {
            id: 1,
            span: Span::new(0, 2),
            value: UnaryExpr {
                kind: UnaryOp::Decrement,
                expr: Box::new(Node {
                    id: 1,
                    span: Span::new(0, 1),
                    value: Expr::Ident(Node {
                        id: 2,
                        span: Span::new(0, 1),
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
