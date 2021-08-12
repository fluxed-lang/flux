use std::error::Error;
use std::str::FromStr;

use cranelift::codegen;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use styxc_ast::{
    Assignment, BinOp, BinOpKind, Block, Declaration, Expr, Literal, LiteralKind, Stmt, StmtKind,
    AST,
};
use target_lexicon::triple;

/// The basic JIT class.
pub struct IrTranslator {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen. Cranelift
    /// separates this from `Module` to allow for parallel compilation, with a
    /// context per thread, though this isn't in the simple demo here.
    ctx: codegen::Context,

    /// The data context, which is to data objects what `ctx` is to functions.
    data_ctx: DataContext,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: ObjectModule,
}

impl IrTranslator {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if cfg!(windows) {
            return Err("windows targets are currently not supported".into());
        }

        let mut flag_builder = settings::builder();
        flag_builder.enable("is_pic").unwrap();

        let isa_builder = isa::lookup(triple!("x86_64-unknown-linux-gnu")).unwrap();
        let isa = isa_builder.finish(settings::Flags::new(flag_builder));

        let builder = ObjectBuilder::new(isa, "name", cranelift_module::default_libcall_names())?;
        let module = ObjectModule::new(builder);

        Ok(Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        })
    }

    /// Translate an AST into LLVM IR.
    pub fn translate(mut self, ast: AST) -> Result<*const u8, Box<dyn Error>> {
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        let mut trans = FunctionTranslator {
            builder,
            // variables,
            module: &mut self.module,
        };

        for stmt in ast.stmts {
            trans.translate_statement(stmt)
        }

        // emit return instruction
        trans.builder.ins().return_(&[]);
        trans.builder.finalize();

        // declare main function
        let id = self
            .module
            .declare_function("main", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(
                id,
                &mut self.ctx,
                &mut codegen::binemit::NullTrapSink {},
                &mut codegen::binemit::NullStackMapSink {},
            )
            .map_err(|e| e.to_string())?;

        // cleanup context and finalize definitions
        self.module.clear_context(&mut self.ctx);
        // return the address of the main function
    }
}

/// A collection of state used for translating from toy-language AST nodes
/// into Cranelift IR.
struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    // variables: HashMap<String, Variable>,
    module: &'a mut ObjectModule,
}

impl<'a> FunctionTranslator<'a> {
    fn translate_statement(&mut self, stmt: Stmt) {
        match stmt.kind {
            StmtKind::Assignment(assign) => self.translate_assignment(assign),
            StmtKind::Declaration(decl) => self.translate_declaration(decl),
            _ => todo!(),
        };
    }

    /// Translate an assignment.
    fn translate_assignment(&mut self, assignment: Assignment) -> Value {
        todo!()
    }

    /// Translate a declaration.
    fn translate_declaration(&mut self, declaration: Vec<Declaration>) {
        for decl in declaration {
            
        }
    }

    /// Translate an expression.
    fn translate_expression(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(literal) => self.translate_literal(literal),
            Expr::BinOp(op) => self.translate_bin_op(op),
            _ => todo!(),
        }
    }

    /// Translate a block.
    fn translate_block(&mut self, block: Block) -> Value {
        let parent = self.builder.current_block().unwrap();
        // create new block and switch to it
        let ir_block = self.builder.create_block();
        self.builder.switch_to_block(ir_block);
        self.builder.seal_block(ir_block);

        for stmt in block.stmts {
            self.translate_statement(stmt)
        }

        self.builder.switch_to_block(parent);
        self.builder.block_params(ir_block)[0]
    }

    /// Translate a literal.
    fn translate_literal(&mut self, literal: Literal) -> Value {
        match literal.kind {
            LiteralKind::Int(int) => self
                .builder
                .ins()
                .iconst(Type::int(32).unwrap(), i64::from(int)),
            _ => todo!(),
        }
    }

    // Translate a binary operation.
    fn translate_bin_op(&mut self, bin_op: BinOp) -> Value {
        let lhs = self.translate_expression(*bin_op.lhs);
        let rhs = self.translate_expression(*bin_op.rhs);

        match bin_op.kind {
            BinOpKind::Eq => self.translate_icmp(IntCC::Equal, lhs, rhs),
            BinOpKind::Ne => self.translate_icmp(IntCC::NotEqual, lhs, rhs),
            BinOpKind::Lt => self.translate_icmp(IntCC::SignedLessThan, lhs, rhs),
            BinOpKind::Gt => self.translate_icmp(IntCC::SignedGreaterThan, lhs, rhs),
            BinOpKind::Le => self.translate_icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
            BinOpKind::Ge => self.translate_icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
            BinOpKind::Add => self.builder.ins().iadd(lhs, rhs),
            BinOpKind::Sub => self.builder.ins().isub(lhs, rhs),
            BinOpKind::Mul => self.builder.ins().imul(lhs, rhs),
            BinOpKind::Div => self.builder.ins().udiv(lhs, rhs),
            _ => todo!(),
        }
    }

    // Translate an integer comparison call.
    fn translate_icmp(&mut self, cmp: IntCC, lhs: Value, rhs: Value) -> Value {
        let c = self.builder.ins().icmp(cmp, lhs, rhs);
        self.builder.ins().bint(Type::int(1).unwrap(), c)
    }
}
