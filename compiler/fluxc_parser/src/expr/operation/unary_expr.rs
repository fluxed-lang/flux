use pest::iterators::Pair;

use fluxc_ast::{Node, UnaryExpr};
use fluxc_errors::CompilerError;

use crate::{Context, Parse, Rule};

impl Parse for UnaryExpr {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
