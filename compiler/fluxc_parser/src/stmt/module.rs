//! Contains data structures for representing imports and exports.

use fluxc_ast::{Export, Import, ImportedSymbol};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Node, Parse, Rule};

impl Parse for ImportedSymbol {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for Import {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for Export {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
