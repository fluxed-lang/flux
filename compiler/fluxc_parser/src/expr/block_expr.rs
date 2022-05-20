use fluxc_ast::{Block, Node, Stmt};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Block {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        debug_assert_eq!(input.as_rule(), Rule::block);
        let node = context.new_empty(input.as_span());
        // parse child statements
        let stmts = input
            .into_inner()
            .map(|inner| Stmt::parse(inner, context))
            .collect::<Result<_, _>>()?;
        Ok(node.fill(Block { stmts }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Block, Expr, Literal, Node, Stmt};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_empty_block() {
        let mut context = Context::default();
        // {}
        let expected = Node { id: 0, span: Span::new(0, 1), value: Block { stmts: vec![] } };
        let result = FluxParser::parse(Rule::block, "{}").unwrap().next().unwrap();
        let result = Block::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // { }
        let expected = Node { id: 1, span: Span::new(0, 2), value: Block { stmts: vec![] } };
        let result = FluxParser::parse(Rule::block, "{ }").unwrap().next().unwrap();
        let result = Block::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
        // {\n\t\n}
        let expected = Node { id: 2, span: Span::new(0, 4), value: Block { stmts: vec![] } };
        let result = FluxParser::parse(Rule::block, "{\n\t\n}").unwrap().next().unwrap();
        let result = Block::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_block_with_stmts() {
        let mut context = Context::default();
        // {
        //     "hello"
        //     "world"
        // }
        let expected = Node {
            id: 0,
            span: Span::new(0, 26),
            value: Block {
                stmts: vec![
                    Node {
                        id: 1,
                        span: Span::new(6, 13),
                        value: Stmt::Expr(Node {
                            id: 2,
                            span: Span::new(6, 12),
                            value: Expr::Literal(Node {
                                id: 3,
                                span: Span::new(6, 12),
                                value: Literal::String("hello".to_string()),
                            }),
                        }),
                    },
                    Node {
                        id: 4,
                        span: Span::new(18, 25),
                        value: Stmt::Expr(Node {
                            id: 5,
                            span: Span::new(18, 24),
                            value: Expr::Literal(Node {
                                id: 6,
                                span: Span::new(18, 24),
                                value: Literal::String("world".to_string()),
                            }),
                        }),
                    },
                ],
            },
        };
        let result = FluxParser::parse(Rule::block, "{\n    \"hello\"\n    \"world\"\n}")
            .unwrap()
            .next()
            .unwrap();
        let result = Block::parse(result, &mut context).unwrap();
        assert_eq!(expected, result);
    }
}
