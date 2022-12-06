//! # fluxc_codegen
//!
//! Handles the generation of code for the Flux compiler.

use std::{
    error::Error,
    sync::{Arc, RwLock},
};

use cranelift::{codegen::Context, frontend::FunctionBuilder, prelude::FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{
    DataContext, FuncId, Linkage, Module, ModuleCompiledFunction, ModuleResult,
};
use fluxc_ast::{FuncDecl, Stmt, AST};
use fluxc_errors::CompilerError;

mod expr;
mod stmt;

/// The context for the module currently being compiled.
pub struct ModuleContext {
    builder_context: FunctionBuilderContext,
    ctx: Context,
    data_ctx: DataContext,
    module: JITModule,
}

impl ModuleContext {
    /// Create a new context for a new module.
    pub fn for_module() -> Arc<RwLock<Self>> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let module = JITModule::new(builder.unwrap());
        Arc::new(RwLock::new(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }))
    }

    /// Enter the given function and create the translation context.
    pub fn declare_function(&mut self, decl: &FuncDecl) -> ModuleResult<FuncId> {
        match decl {
            FuncDecl::Local { ident, params, body, ret_ty } => {
                let signature = self.module.make_signature();
                self.module.declare_function(&ident.value, Linkage::Local, &signature)
            }
            FuncDecl::Export { ident, params, body, ret_ty } => {
                let signature = self.module.make_signature();
                self.module.declare_function(&ident.value, Linkage::Export, &signature)
            }
            FuncDecl::External { ident, params, ret_ty } => {
                let signature = self.module.make_signature();
                self.module.declare_function(&ident.value, Linkage::Import, &signature)
            }
        }
    }

    /// Define the given function.
    pub fn define_function(
        &mut self,
        decl: &FuncDecl,
        id: FuncId,
    ) -> ModuleResult<ModuleCompiledFunction> {
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let mut ctx = TranslationContext { builder, module: Arc::new(RwLock::new(self)) };

        // translate function body
        decl.translate(&mut ctx);

        self.module.define_function(id, &mut self.ctx)
    }
}

/// The context in which code generation is occuring.
pub struct TranslationContext<'a> {
    pub builder: FunctionBuilder<'a>,
    pub module: Arc<RwLock<ModuleContext>>,
}

/// Trait implemented by types that can generate code.
pub trait Translate {
    /// Emit code for this type.
    fn translate<'a>(&self, ctx: &mut TranslationContext<'a>) -> Result<(), CompilerError>;
}

/// Handle the AST code generation.
#[tracing::instrument]
pub fn codegen(ast: AST) -> Result<(), Box<dyn Error>> {
    let mut module_ctx = ModuleContext::for_module();

    // collect all function declarations
    let func_decls = ast
        .stmts
        .iter()
        .filter(|stmt| matches!(stmt.value, Stmt::FuncDecl(_)))
        .map(|node| match &node.value {
            Stmt::FuncDecl(decl) => &decl.value,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    // declare functions
    let mut funcs = Vec::with_capacity(func_decls.len());
    for func_decl in func_decls {
        funcs.push((func_decl, module_ctx.declare_function(func_decl)?));
    }
    // define functions
    for (decl, id) in funcs {
        module_ctx.define_function(decl, id)?;
    }

    Ok(())
}
