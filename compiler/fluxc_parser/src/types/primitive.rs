use fluxc_ast::Primitive;
use pest::iterators::Pair;

use crate::{Context, PResult, Parse};

impl Parse for Primitive {
    fn parse<'i>(
        input: Pair<'i, crate::parser::Rule>,
        ctx: &mut Context,
    ) -> PResult<Self> {
        todo!()
    }
}
