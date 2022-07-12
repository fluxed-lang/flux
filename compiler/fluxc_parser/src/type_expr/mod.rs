use fluxc_ast::{Node, TypeExpr};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Parse, Rule};

mod primitive;

impl Parse for TypeExpr {
    fn parse<'i>(
        _input: Pair<'i, Rule>,
        _ctx: &mut crate::Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
