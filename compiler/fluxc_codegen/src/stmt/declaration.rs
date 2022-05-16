use fluxc_ast::Declaration;
use fluxc_errors::CompilerError;

use crate::{TranslationContext, Translate};

impl Translate for Declaration {
	fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
		todo!()
	}
}
