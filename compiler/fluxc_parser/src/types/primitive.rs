use fluxc_ast::{Node, Primitive};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse};

impl Parse for Primitive {
    fn parse<'i>(
        input: Pair<'i, crate::parser::Rule>,
        ctx: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
