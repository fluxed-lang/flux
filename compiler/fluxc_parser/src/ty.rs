use fluxc_ast::Node;
use fluxc_errors::CompilerError;
use fluxc_types::Type;
use pest::iterators::Pair;

use crate::{Context, Parse};

impl Parse for Type {
    fn parse<'i>(
        input: Pair<'i, crate::parser::Rule>,
        ctx: &mut Context,
    ) -> PResult<Self> {
        todo!()
    }
}
