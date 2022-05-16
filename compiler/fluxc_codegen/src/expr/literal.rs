use fluxc_ast::Literal;
use fluxc_errors::CompilerError;
use fluxc_types::{Operation, Primitive, Type, Typed, Unify};

use crate::{TranslationContext, Translate};

impl Translate for Literal {
	fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError> {
		todo!()
	}
}
