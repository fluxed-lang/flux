use fluxc_ast::Match;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Match {
    fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
