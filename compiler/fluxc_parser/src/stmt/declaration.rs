use fluxc_ast::{Declaration, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse};

impl Parse for Declaration {
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
