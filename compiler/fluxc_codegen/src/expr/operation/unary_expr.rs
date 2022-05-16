use fluxc_errors::CompilerError;

use crate::{TranslationContext, Translate};

impl Translate for UnaryExpr {
	fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
		todo!()
	}
}
