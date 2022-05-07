use pest::iterators::Pair;

use fluxc_ast::{Block, Node};
use fluxc_errors::CompilerError;

use crate::{Context, Parse, Rule};

impl Parse for Block {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
