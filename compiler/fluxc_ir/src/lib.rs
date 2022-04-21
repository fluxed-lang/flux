use std::{collections::HashMap, error::Error};

use cranelift::{
    codegen,
    frontend::{FunctionBuilder, FunctionBuilderContext, Variable},
    prelude::{
        settings, types, AbiParam, Configurable, EntityRef, InstBuilder, IntCC, Signature, Type,
        Value,
    },
};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use log::{debug, trace};
use styxc_ast::{
    control::{If, Loop},
    func::FuncCall,
    operations::{Assignment, AssignmentKind, BinaryExpr, BinaryOp},
    Declaration, Expr, Ident, Literal, Literal, Node, Stmt, AST,
};
use styxc_walker::Walker;

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
        let mut trans = FunctionTranslator::new(builder, &mut self.module, &mut self.data_ctx);
        trans.translate_stmts(ast.stmts);
        trace!("Finalizing builder...");
        trans.builder.ins().return_(&vec![]);
        trans.builder.finalize();
        let display = Some(trans.builder.func.display().to_string());
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

    // /// Create a zero-initialized data section.
    // fn create_data(&mut self, name: &str, contents: Vec<u8>) -> Result<&[u8], String> {
    //     self.data_ctx.define(contents.into_boxed_slice());
    //     let id = self
    //         .module
    //         .declare_data(name, Linkage::Export, true, false)
    //         .map_err(|e| e.to_string())?;
    //     self.module
    //         .define_data(id, &self.data_ctx)
    //         .map_err(|e| e.to_string())?;
    //     self.data_ctx.clear();
    //     self.module.finalize_definitions();
    //     let buffer = self.module.get_finalized_data(id);
    //     Ok(unsafe { from_raw_parts(buffer.0, buffer.1) })
    // }
}

fn type_to_ir_type(module: &dyn Module, ty: styxc_types::Type) -> Option<Type> {
    use styxc_types::Type::*;
    // return none if unit type
    if matches!(ty, styxc_types::Type::Unit) {
        return None;
    }
    Some(match ty {
        Int => types::I64,
        Float => types::F64,
        Bool => types::B1,
        Char => types::I32,
        String => module.target_config().pointer_type(),
        Tuple(_) => todo!(),
        Array(_) => todo!(),
        Map(_, _) => todo!(),
        Set(_) => todo!(),
        Optional(_) => todo!(),
        Union(_) => unreachable!(),
        Intersection(_) => todo!(),
        Circular(_) => todo!(),
        Unit => todo!(),
        Infer => panic!("failed to infer type"),
        Never => todo!(),
        Func(_, _) => todo!(),
        Reference(_) => todo!(),
    })
}

/// Utility struct for generating functions.
struct FunctionTranslator<'a> {
    builder: FunctionBuilder<'a>,
    variables: HashMap<String, Variable>,
    module: &'a mut JITModule,
    data_ctx: &'a mut DataContext,
    index: usize,
    walker: Walker,
}

impl<'a> FunctionTranslator<'a> {
    /// Create a new function translator using the specified Cranelift function builder and JIT module.
    pub fn new(
        builder: FunctionBuilder<'a>,
        module: &'a mut JITModule,
        data_ctx: &'a mut DataContext,
    ) -> Self {
        Self {
            builder,
            module,
            data_ctx,
            variables: HashMap::new(),
            index: 0,
            walker: Walker::new(),
        }
    }

    fn resolve_var(&mut self, ident: &Ident) -> Option<Value> {
        match self.variables.get(&ident.inner) {
            Some(var) => Some(self.builder.use_var(*var)),
            None => None,
        }
    }

    /// Translate and build statements.
    fn translate_stmts(&mut self, stmts: Vec<Node<Stmt>>) {
        self.walker.declare_all_in_stmts(&stmts);
        for stmt in stmts {
            self.translate_stmt(stmt.value);
        }
    }

    /// Translate and build a statement.
    fn translate_stmt(&mut self, stmt: Stmt) {
        self.walker.next_stmt(&stmt);
        trace!("TRANSLATE Stmt");
        use Stmt::*;
        match stmt {
            Declaration(decl) => decl
                .into_iter()
                .for_each(|decl| self.translate_declaration(decl.value)),
            Assignment(assign) => self.translate_assignment(assign.value),
            Loop(loop_node) => self.translate_loop(loop_node.value),
            If(if_stmt) => self.translate_if(if_stmt.value),
            // FuncCall(call) => { self.translate_func_call(call.value); },
            ExternFunc(extern_func) => {
                // create function signature
                let mut sig = Signature::new(self.module.target_config().default_call_conv);
                // declare parameter types
                extern_func
                    .value
                    .args
                    .into_iter()
                    .map(|arg| type_to_ir_type(self.module, arg.value.ty).unwrap())
                    .map(|ty| AbiParam::new(ty))
                    .for_each(|param| sig.params.push(param));
                // declare return type
                let ret_ty = type_to_ir_type(
                    self.module,
                    match extern_func.value.ty {
                        styxc_types::Type::Func(_, ret_ty) => *ret_ty,
                        _ => panic!("ExternFunc should have a function type"),
                    },
                );
                // declare the return type if it exists
                if let Some(ret_ty) = ret_ty {
                    sig.returns.push(AbiParam::new(ret_ty));
                }
                // declare function
                self.module
                    .declare_function(&extern_func.value.ident.value.inner, Linkage::Import, &sig)
                    .unwrap();
            }
            FuncDecl(_) => todo!(),
            Return(expr) => {
                // translate the expression and return
                let val = self.translate_expr(expr.value);
                self.builder.ins().return_(&vec![val]);
            }
            FuncCall(call) => {
                self.translate_func_call(call.value);
            }
            _ => todo!(),
        }
    }

    /// Translate an expression block.
    fn translate_expr(&mut self, expr: Expr) -> Value {
        trace!("TRANSLATE Expr");
        use Expr::*;
        match expr {
            Literal(literal) => self.translate_literal(literal.value),
            Ident(ident) => self
                .builder
                .use_var(*self.variables.get(&ident.value.inner).unwrap()),
            BinaryExpr(bin_op) => self.translate_bin_op(bin_op.value),
            Block(_) => todo!(),
            FuncCall(func_call) => {
                if matches!(func_call.value.return_ty, styxc_types::Type::Unit) {
                    panic!("")
                }
                self.translate_func_call(func_call.value).unwrap()
            }
        }
    }

    fn translate_literal(&mut self, literal: Literal) -> Value {
        trace!("TRANSLATE Literal");
        use Literal::*;
        match literal.kind {
            Int(val) => self
                .builder
                .ins()
                .iconst(self.module.target_config().pointer_type(), val),
            Float(val) => self.builder.ins().f64const(val),
            String(contents) => {
                // define the data in the context
                self.data_ctx.define(contents.as_bytes().into());
                let data = self.module.declare_anonymous_data(false, false).unwrap();
                // define and declare the data to the module
                self.module.define_data(data, self.data_ctx).unwrap();
                self.data_ctx.clear();
                self.module.finalize_definitions();
                // get the address of the data and return it
                let (addr, _) = self.module.get_finalized_data(data);
                let pointer = self.module.target_config().pointer_type();
                self.builder.ins().iconst(pointer, addr as i64)
            }
            Char(val) => self.builder.ins().iconst(types::I32, val as i64),
            Bool(val) => self.builder.ins().bconst(types::B1, val),
        }
    }

    /// Translate a declaration statement.
    fn translate_declaration(&mut self, decl: Declaration) {
        trace!("TRANSLATE Declaration");
        let var = Variable::new(self.index);
        self.index += 1;
        self.variables.insert(decl.ident.value.inner, var);
        let val = self.translate_expr(decl.value.value);
        self.builder
            .declare_var(var, type_to_ir_type(self.module, decl.ty).unwrap());
        self.builder.def_var(var, val)
    }

    /// Translate an assignment statement.
    fn translate_assignment(&mut self, assign: Assignment) {
        trace!("TRANSLATE Assignment");

        use AssignmentKind::*;

        let val = self.resolve_var(&assign.ident.value).unwrap();
        let rhs = self.translate_expr(assign.value.value);

        let new_value = match assign.kind {
            Assign => rhs,
            ShlAssign => self.builder.ins().rotr(val, rhs),
            ShrAssign => self.builder.ins().rotl(val, rhs),
            AndAssign => self.builder.ins().band(val, rhs),
            OrAssign => self.builder.ins().bor(val, rhs),
            XorAssign => self.builder.ins().bxor(val, rhs),
            AddAssign => self.builder.ins().iadd(val, rhs),
            SubAssign => self.builder.ins().isub(val, rhs),
            MulAssign => self.builder.ins().imul(val, rhs),
            DivAssign => self.builder.ins().sdiv(val, rhs),
            ModAssign => self.builder.ins().srem(val, rhs),
        };
        let variable = self.variables.get(&assign.ident.value.inner).unwrap();
        self.builder.def_var(*variable, new_value);
    }

    /// Translate a loop statement.
    fn translate_loop(&mut self, loop_node: Loop) {
        trace!("TRANSLATE Loop");
        self.walker.enter_block(&loop_node.block.value);
        let body_block = self.builder.create_block();
        let exit_block = self.builder.create_block();
        // create jump instruction and jump to block.
        self.builder.ins().jump(body_block, &[]);
        self.builder.switch_to_block(body_block);
        // translate loop statements
        self.translate_stmts(loop_node.block.value.stmts);
        self.builder.ins().jump(body_block, &[]);
        // seal and switch to exit block
        self.builder.switch_to_block(exit_block);
        self.builder.seal_block(body_block);
        self.builder.seal_block(exit_block);
    }

    // Translate a binary operation.
    fn translate_bin_op(&mut self, bin_op: BinaryExpr) -> Value {
        let lhs = self.translate_expr(bin_op.lhs.value);
        let rhs = self.translate_expr(bin_op.rhs.value);
        use BinaryOp::*;
        match bin_op.kind {
            Plus => self.builder.ins().iadd(lhs, rhs),
            Minus => self.builder.ins().isub(lhs, rhs),
            Mul => self.builder.ins().imul(lhs, rhs),
            Div => self.builder.ins().udiv(lhs, rhs),
            Mod => self.builder.ins().srem(lhs, rhs),
            BitwiseAnd => self.builder.ins().band(lhs, rhs),
            BitwiseOr => self.builder.ins().bor(lhs, rhs),
            BitwiseXor => self.builder.ins().bxor(lhs, rhs),
            LogicalAnd => todo!(),
            LogicalOr => todo!(),
            Shl => self.builder.ins().ishl(lhs, rhs),
            Shr => self.builder.ins().sshr(lhs, rhs),
            Eq | Ne | Lt | Gt | Le | Ge => self.translate_icmp(bin_op.kind, lhs, rhs),
            Assign | PlusEq | MinusEq | MulEq | DivEq | ModEq | BitwiseAndEq | BitwiseOrEq
            | BitwiseXorEq | ShlEq | ShrEq => todo!(),
        }
    }

    /// Translate an icmp comparison code.
    fn translate_icmp(&mut self, op: BinaryOp, lhs: Value, rhs: Value) -> Value {
        use BinaryOp::*;
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
        self.walker.enter_block(&if_stmt.block.value);

        let condition_value = self.translate_expr(if_stmt.expr.value);
        let then_block = self.builder.create_block();
        let merge_block = self.builder.create_block();
        // test if condition and conditionally branch
        self.builder.ins().brz(condition_value, merge_block, &[]);
        // go to then block if condition is true
        self.builder.ins().jump(then_block, &[]);
        self.builder.switch_to_block(then_block);
        self.builder.seal_block(then_block);
        // output statements into block.
        self.translate_stmts(if_stmt.block.value.stmts);
        // jump back to merging block
        self.builder.ins().jump(merge_block, &[]);
        // switch to the merge block for subsequent statements.
        self.builder.switch_to_block(merge_block);
        self.builder.seal_block(merge_block);
    }

    fn translate_func_call(&mut self, call: FuncCall) -> Option<Value> {
        let mut sig = self.module.make_signature();
        let func = self
            .walker
            .lookup_function(&call.ident.value.inner)
            .unwrap();

        // Add a parameter for each argument.
        let arg_tys;
        let ret_ty;
        if let styxc_types::Type::Func(func_arg_tys, func_ret_ty) = &func.ty {
            arg_tys = func_arg_tys.clone();
            ret_ty = *func_ret_ty.clone();
        } else {
            panic!("function type was not a function")
        }
        // iterate over arguments and add to signature
        for arg in arg_tys {
            sig.params
                .push(AbiParam::new(type_to_ir_type(self.module, arg).unwrap()));
        }
        // push return signature if there is one
        if let Some(ret_ty) = type_to_ir_type(self.module, ret_ty) {
            sig.returns.push(AbiParam::new(ret_ty));
        }
        // declare the function
        let callee = self
            .module
            .declare_function(&call.ident.value.inner, Linkage::Import, &sig)
            .expect("problem declaring function");
        let local_callee = self
            .module
            .declare_func_in_func(callee, &mut self.builder.func);

        let mut arg_values = Vec::new();
        for arg in call.args {
            arg_values.push(self.translate_expr(arg.value))
        }
        let call = self.builder.ins().call(local_callee, &arg_values);
        self.builder.inst_results(call);
        None
    }
}
