use pest::iterators::Pair;

use fluxc_ast::{BinaryExpr, Node};
use fluxc_errors::CompilerError;

use crate::{Parse, Rule, Context};

impl Parse for BinaryExpr {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
