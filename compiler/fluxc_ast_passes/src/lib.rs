use std::{error::Error, time::Instant};

use fluxc_ast::AST;
use log::debug;

mod types;

/// Perform all defined AST passes on the given AST.
pub fn perform_ast_passes(ast: &mut AST) -> Result<(), Box<dyn Error>> {
    debug!("Start AST passes...");
    let start = Instant::now();
    debug!("Performing AST type pass...");
    types::perform_ast_type_pass(ast)?;
    debug!("End AST passes - took {}ms", start.elapsed().as_millis());
    Ok(())
}
