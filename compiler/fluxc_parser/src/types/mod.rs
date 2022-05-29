use fluxc_ast::{Node, TypeExpr};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Parse, Rule};

mod primitive;

impl Parse for TypeExpr {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        ctx: &mut crate::Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
