use fluxc_ast::FuncDecl;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for FuncDecl {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
