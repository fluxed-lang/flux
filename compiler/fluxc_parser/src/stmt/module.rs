//! Contains data structures for representing imports and exports.

use pest::iterators::Pair;

use fluxc_ast::{Export, Import, ImportedSymbol};
use fluxc_errors::CompilerError;

use crate::{Context, Node, Parse, Rule};

impl Parse for ImportedSymbol {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for Import {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for Export {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
