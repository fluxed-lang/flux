use fluxc_types::Type;
use pest::iterators::Pair;

use crate::{Context, PResult, Parse};

impl Parse for Type {
    fn parse<'i>(_input: Pair<'i, crate::parser::Rule>, _ctx: &mut Context) -> PResult<Self> {
        todo!()
    }
}
