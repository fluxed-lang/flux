use fluxc_ast::Block;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Block {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
