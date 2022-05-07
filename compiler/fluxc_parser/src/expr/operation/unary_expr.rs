use pest::iterators::Pair;

use fluxc_ast::{UnaryExpr, Node};
use fluxc_errors::CompilerError;

use crate::{Parse, Context, Rule};

impl Parse for UnaryExpr {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
