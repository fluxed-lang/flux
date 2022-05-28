use fluxc_ast::While;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for While {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
