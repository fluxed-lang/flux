use std::{error::Error, fs::File, io::Read, path::Path};

use log::{debug, error};

/// Enum of possible compiler modes.
pub enum Mode<'i> {
    /// Represents the Just-In-Time compile mode.
    JIT,
    /// Represents the Ahead-Of-Tme compile mode.
    AOT(&'i Path),
}

/// Compile the target input string into memory.
pub fn compile_to_mem(input: String) -> Result<fn() -> (), Box<dyn Error>> {
    // 1. Parse input source
    let mut parser = styxc_parser::StyxParser::default();
    let ast = parser.build(&input)?;

    Ok(|| ())
}

/// Compile the target input string into memory and execute it immediately.
fn compile_and_execute(input: String) -> Result<(), Box<dyn Error>> {
    match compile_to_mem(input) {
        Ok(mem) => Ok(mem()),
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
