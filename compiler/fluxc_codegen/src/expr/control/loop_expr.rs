use fluxc_ast::Loop;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Loop {
    fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
