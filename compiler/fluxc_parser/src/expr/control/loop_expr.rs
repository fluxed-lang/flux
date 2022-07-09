use fluxc_ast::{Block, Loop};
use pest::iterators::Pair;

use crate::{Context, Parse, Rule, PResult};

impl Parse for Loop {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::loop_stmt);
        let node = ctx.new_empty(input.as_span());
        let inner = input.into_inner();
        Ok(node.fill(Loop { name: None, block: Block::parse(inner.last().unwrap(), ctx)? }))
    }
}

#[cfg(test)]
mod tests {
    use fluxc_ast::{Block, Loop, Node};
    use fluxc_span::Span;
    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{parser::FluxParser, Context, Parse, Rule};

    #[test]
    fn parse_loop_expr() {
        let mut context = Context::default();
        // loop {}
        let expected = Node {
            id: 0,
            span: Span::new(0, 6),
            value: Loop {
                name: None,
                block: Node { id: 1, span: Span::new(5, 6), value: Block { stmts: vec![] } },
            },
        };
        let actual = Loop::parse(
            FluxParser::parse(Rule::loop_stmt, "loop {}").unwrap().next().unwrap(),
            &mut context,
        )
        .unwrap();
        assert_eq!(actual, expected);
    }
}
