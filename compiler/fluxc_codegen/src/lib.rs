//! # fluxc_codegen
//!
//! Handles the generation of code for the Flux compiler.

use cranelift::{
    codegen::Context,
    frontend::{FunctionBuilder, FunctionBuilderContext},
};
use cranelift_jit::JITModule;
use cranelift_module::DataContext;
use fluxc_errors::CompilerError;

mod expr;
mod stmt;

/// The code generation handler.
struct CodeGenerator {
    builder_ctx: FunctionBuilderContext,
    ctx: Context,
    data_ctx: DataContext,
    module: JITModule,
}

/// The context in which code generation is occuring.
pub struct TranslationContext<'a> {
    pub builder: FunctionBuilder<'a>,
    pub module: &'a mut JITModule,
}

/// Trait implemented by types that can generate code.
pub trait Translate {
    /// Emit code for this type.
    fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError>;
}
