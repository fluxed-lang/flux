use fluxc_ast::{Literal, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Literal {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
