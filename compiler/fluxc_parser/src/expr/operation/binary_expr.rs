use fluxc_ast::{BinaryExpr, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for BinaryExpr {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
