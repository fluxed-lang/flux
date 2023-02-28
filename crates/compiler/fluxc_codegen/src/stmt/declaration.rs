use fluxc_ast::Declaration;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for Declaration {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
