use fluxc_ast::Node;
use fluxc_errors::CompilerError;
use fluxc_types::{Primitive, Type};
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Primitive {
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {}
}

impl Parse for Type {
    fn parse<'i>(input: Pair<'i, Rule>, ctx: &mut Context) -> Result<Node<Self>, CompilerError> {
        let node = ctx.new_empty(input.as_span());
        let inner = input.into_inner().next().unwrap();

        Ok(match inner.as_rule() {
            Rule::type_literal => node.fill(Type::Primitive(Primitive::parse(inner, ctx)?)),
        })
    }
}
