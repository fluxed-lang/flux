use pest::iterators::Pair;

use fluxc_ast::Assignment;
use fluxc_errors::CompilerError;

use crate::{Node, Parse, Context, Rule};

impl Parse for Assignment {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
