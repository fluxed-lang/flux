use fluxc_ast::BinaryExpr;
use fluxc_errors::CompilerError;

use crate::{Translate, TranslationContext};

impl Translate for BinaryExpr {
    fn translate<'a>(&self, _ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
        todo!()
    }
}
