use fluxc_ast::Primitive;
use pest::iterators::Pair;

use crate::{Context, PResult, Parse};

impl Parse for Primitive {
    fn parse<'i>(
        input: Pair<'i, crate::sealed::Rule>,
        ctx: &mut Context,
    ) -> PResult<Self> {
        todo!()
    }
}
