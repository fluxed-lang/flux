use fluxc_ast::Primitive;
use pest::iterators::Pair;

use crate::{Context, PResult, Parse};

impl Parse for Primitive {
    fn parse<'i>(_input: Pair<'i, crate::sealed::Rule>, _ctx: &mut Context) -> PResult<Self> {
        todo!()
    }
}
