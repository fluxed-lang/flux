use fluxc_ast::{Loop, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Loop {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
