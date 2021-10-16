use std::{collections::HashMap, error::Error, slice::from_raw_parts};

use cranelift::{
    codegen,
    frontend::{FunctionBuilder, FunctionBuilderContext, Variable},
    prelude::{settings, types, Configurable, EntityRef, InstBuilder, IntCC, Type, Value},
};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use log::{debug, trace};
use styxc_ast::{
    Assignment, AssignmentKind, BinOp, BinOpKind, Declaration, Expr, Ident, If, Literal,
    LiteralKind, Loop, Stmt, StmtKind, Var, AST,
};

/// Represents a variable in the current stack.
struct IrVar(Var, Variable);

/// Root-level IR translator.
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
    module: JITModule,
}

impl Default for IrTranslator {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        // On at least AArch64, "colocated" calls use shorter-range relocations,
        // which might not reach all definitions; we can't handle that here, so
        // we require long-range relocation types.
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder.finish(settings::Flags::new(flag_builder));
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        // declare builder and module
        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            data_ctx: DataContext::new(),
            module,
        }
    }
}

impl IrTranslator {
    /// Build the AST into runnable code.
    pub fn build(&mut self, ast: AST) -> Result<(*const u8, Option<String>), Box<dyn Error>> {
        debug!("Building IR from AST...");
        // translate statements
        trace!("Creating builder and entry block...");
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_block = builder.create_block();
        // switch to and seal block - this is the main function, so has no predecessors
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);
        // instantiate function builder and build.
        trace!("Instantiating function builder...");
        let mut trans = FunctionTranslator::new(builder, &mut self.module);
        trans.translate_stmts(ast.stmts);
        trace!("Finalizing builder...");
        trans.builder.ins().return_(&vec![]);
        trans.builder.finalize();
        let display = Some(trans.builder.func.display(None).to_string());
        // declare the main function
        trace!("Declaring main function...");
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
        // finish up
        trace!("Clear context and finalize definitions...");
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();

        // return address of function
        let code = self.module.get_finalized_function(id);
        Ok((code, display))
    }

    /// Create a zero-initialized data section.
    fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
        self.data_ctx.define(contents.into_boxed_slice());
        let id = self
            .module
            .declare_data(name, Linkage::Export, true, false)
            .map_err(|e| e.to_string())?;
        self.module
            .define_data(id, &self.data_ctx)
            .map_err(|e| e.to_string())?;
        self.data_ctx.clear();
        self.module.finalize_definitions();
        let buffer = self.module.get_finalized_data(id);
        Ok(unsafe { from_raw_parts(buffer.0, buffer.1) })
    }
}

fn type_to_ir_type(ty: styxc_types::Type) -> Type {
    use styxc_types::Type::*;
    match ty {
        Int => types::I64,
        Float => types::F64,
        Bool => types::B1,
        Char => types::I32,
        String => todo!("string type not supported"),
        Tuple(_) => todo!(),
        Array(_) => todo!(),
        Map(_, _) => todo!(),
        Set(_) => todo!(),
        Optional(_) => todo!(),
        Union(_) => todo!(),
        Intersection(_) => todo!(),
        Circular(_) => todo!(),
        Unit => todo!(),
        Infer => panic!("failed to infer type"),
        Never => todo!(),
    }
}

/// Utility struct for generating functions.
struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
    index: usize,
}

impl<'a> FunctionTranslator<'a> {
    /// Create a new function translator using the specified Cranelift function builder and JIT module.
    pub fn new(builder: FunctionBuilder<'a>, module: &'a mut JITModule) -> Self {
        Self {
            builder,
            module,
            variables: HashMap::new(),
            index: 0,
        }
    }

    fn resolve_var(&mut self, ident: &Ident) -> Option<Value> {
        match self.variables.get(&ident.name) {
            Some(var) => Some(self.builder.use_var(*var)),
            None => None,
        }
    }

    /// Translate and build statements.
    fn translate_stmts(&mut self, stmts: Vec<Stmt>) {
        for stmt in stmts {
            self.translate_stmt(stmt);
        }
    }

    /// Translate and build a statement.
    fn translate_stmt(&mut self, stmt: Stmt) {
        trace!("TRANSLATE Stmt");
        use StmtKind::*;
        match stmt.kind {
            Declaration(decl) => decl
                .into_iter()
                .for_each(|decl| self.translate_declaration(decl)),
            Assignment(assign) => self.translate_assignment(assign),
            Loop(loop_node) => self.translate_loop(loop_node),
            If(if_stmt) => self.translate_if(if_stmt),
        }
    }

    /// Translate an expression block.
    fn translate_expr(&mut self, expr: Expr) -> Value {
        trace!("TRANSLATE Expr");
        use Expr::*;
        match expr {
            Literal(literal) => self.translate_literal(literal),
            Ident(ident) => self
                .builder
                .use_var(*self.variables.get(&ident.name).unwrap()),
            BinOp(bin_op) => self.translate_bin_op(bin_op),
            Block(block) => todo!(),
        }
    }

    fn translate_literal(&mut self, literal: Literal) -> Value {
        trace!("TRANSLATE Literal");
        use LiteralKind::*;
        match literal.kind {
            Int(val) => self
                .builder
                .ins()
                .iconst(self.module.target_config().pointer_type(), val),
            Float(val) => self.builder.ins().f64const(val),
            String(_) => todo!("no"),
            Char(val) => self.builder.ins().iconst(types::I32, val as i64),
            Bool(val) => self.builder.ins().bconst(types::B1, val),
        }
    }

    /// Translate a declaration statement.
    fn translate_declaration(&mut self, decl: Declaration) {
        trace!("TRANSLATE Declaration");
        let var = Variable::new(self.index);
        self.index += 1;
        self.variables.insert(decl.ident.name, var);
        let val = self.translate_expr(decl.value);
        self.builder.declare_var(var, type_to_ir_type(decl.ty));
        self.builder.def_var(var, val)
    }

    /// Translate an assignment statement.
    fn translate_assignment(&mut self, assign: Assignment) {
        trace!("TRANSLATE Assignment");

        use AssignmentKind::*;
        let new_value = match assign.kind {
            Assign => self.translate_expr(assign.value),
            ShlAssign => todo!(),
            ShrAssign => todo!(),
            AndAssign => todo!(),
            OrAssign => todo!(),
            XorAssign => todo!(),
            AddAssign => {
                let val = self.resolve_var(&assign.ident).unwrap();
                let to_add = self.translate_expr(assign.value);
                self.builder.ins().iadd(val, to_add)
            }
            SubAssign => todo!(),
            MulAssign => todo!(),
            DivAssign => todo!(),
            ModAssign => todo!(),
        };
        let variable = self.variables.get(&assign.ident.name).unwrap();
        self.builder.def_var(*variable, new_value);
    }

    /// Translate a loop statement.
    fn translate_loop(&mut self, loop_node: Loop) {
        trace!("TRANSLATE Loop");
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();
        // create jump instruction and jump to block.
        self.builder.ins().jump(body_block, &[]);
        self.builder.switch_to_block(body_block);
        // translate loop statements
        self.translate_stmts(loop_node.block.stmts);
        self.builder.ins().jump(body_block, &[]);
        // seal and switch to exit block
        self.builder.switch_to_block(exit_block);
        self.builder.seal_block(body_block);
        self.builder.seal_block(exit_block);
    }

    // Translate a binary operation.
    fn translate_bin_op(&mut self, bin_op: BinOp) -> Value {
        let lhs = self.translate_expr(*bin_op.lhs);
        let rhs = self.translate_expr(*bin_op.rhs);
        use BinOpKind::*;
        match bin_op.kind {
            Add => self.builder.ins().iadd(lhs, rhs),
            Sub => self.builder.ins().isub(lhs, rhs),
            Mul => self.builder.ins().imul(lhs, rhs),
            Div => self.builder.ins().udiv(lhs, rhs),
            Mod => self.builder.ins().srem(lhs, rhs),
            And => self.builder.ins().band(lhs, rhs),
            Or => self.builder.ins().bor(lhs, rhs),
            Xor => self.builder.ins().bxor(lhs, rhs),
            LogAnd => todo!(),
            LogOr => todo!(),
            Shl => self.builder.ins().ishl(lhs, rhs),
            Shr => self.builder.ins().sshr(lhs, rhs),
            Eq | Ne | Lt | Gt | Le | Ge => self.translate_icmp(bin_op.kind, lhs, rhs),
        }
    }

    /// Translate an icmp comparison code.
    fn translate_icmp(&mut self, op: BinOpKind, lhs: Value, rhs: Value) -> Value {
        use BinOpKind::*;
        match op {
            Eq => self.builder.ins().icmp(IntCC::Equal, lhs, rhs),
            Ne => self.builder.ins().icmp(IntCC::NotEqual, lhs, rhs),
            Lt => self.builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs),
            Gt => self.builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs),
            Le => self
                .builder
                .ins()
                .icmp(IntCC::SignedLessThanOrEqual, lhs, rhs),
            Ge => self
                .builder
                .ins()
                .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs),
            _ => panic!("bad icmp type"),
        }
    }

    /// Translate an if statement in to code.
    fn translate_if(&mut self, if_stmt: If) {
        let condition_value = self.translate_expr(if_stmt.expr);
        let then_block = self.builder.create_block();
        let merge_block = self.builder.create_block();
        // test if condition and conditionally branch
        self.builder.ins().brz(condition_value, merge_block, &[]);
        // go to then block if condition is true
        self.builder.ins().jump(then_block, &[]);
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        // output statements into block.
        self.translate_stmts(if_stmt.block.stmts);
        // jump back to merging block
        self.builder.ins().jump(merge_block, &[]);
        // switch to the merge block for subsequent statements.
        self.builder.switch_to_block(merge_block);
        self.builder.seal_block(merge_block);
    }
}
