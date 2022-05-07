use pest::iterators::Pair;

use fluxc_ast::{Node, Loop};
use fluxc_errors::CompilerError;

use crate::{Parse, Rule, Context};

impl Parse for Loop {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
