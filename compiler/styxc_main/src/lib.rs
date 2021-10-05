use std::{error::Error, fs::File, io::Read, mem, path::Path, time::Instant};

use log::{debug, info};
use styxc_ast::ASTValidator;

/// Enum of possible compiler modes.
pub enum Mode<'i> {
    /// Represents the Just-In-Time compile mode.
    JIT,
    /// Represents the Ahead-Of-Tme compile mode.
    AOT(&'i Path),
}

/// Compile the target input string into memory.
pub fn compile_to_mem(input: String) -> Result<fn() -> u32, Box<dyn Error>> {
    // 1. Parse input source
    let mut parser = styxc_parser::StyxParser::default();
    let ast = parser.build(&input)?;
    // 2. Run AST validation on the AST
    ASTValidator::default().walk(&ast)?;
    // 3. Generate IR
    let (pointer, display) = styxc_ir::IrTranslator::default().build(ast)?;
    let code_fn;
    unsafe {
        code_fn = mem::transmute::<_, fn() -> u32>(pointer);
    }
    Ok(code_fn)
}

/// Compile the target input string into memory and execute it immediately.
fn compile_and_execute(input: String) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    match compile_to_mem(input) {
        Ok(mem) => {
            debug!("Compiled in {}ms", now.elapsed().as_millis());
            let res = mem();
            info!("program output was {}", res);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Compile the target input string into an executable binary.
pub fn compile_to_binary<P: AsRef<Path>>(input: String, dest: P) -> Result<(), Box<dyn Error>> {
    panic!("unsupported compiler mode");
}

/// Compile the target file using the given compiler mode.
pub fn compile<P: AsRef<Path>>(target: P, mode: Mode) -> Result<(), Box<dyn Error>> {
    debug!("Compiling {:?}", target.as_ref());

    let mut file = match File::open(target) {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    };

    match mode {
        Mode::AOT(dest) => compile_to_binary(buf, dest),
        Mode::JIT => compile_and_execute(buf),
    }
}
