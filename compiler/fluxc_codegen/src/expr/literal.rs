use fluxc_ast::Literal;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Literal {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
