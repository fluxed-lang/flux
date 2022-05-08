use fluxc_ast::{Conditional, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Conditional {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
