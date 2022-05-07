use pest::iterators::Pair;

use fluxc_ast::{Conditional, Node};
use fluxc_errors::CompilerError;

use crate::{Parse, Rule, Context};

impl Parse for Conditional {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
