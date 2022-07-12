//! Defines AST data structures and types for representing Flux code at compile
//! time.
//!
//! The AST is split into two primary modules:
//! - `fluxc_ast::expr`: Contains the expression AST data structures, or things
//!   that have types.
//! - `fluxc_ast::stmt`: Contains the statement AST data structures.
//!
//! The AST is built from the parser, iterated over by reducers in the
//! `fluxc_ast_passes` crate, before being sent to `fluxc_codegen` and turned
//! into valid LLVM code.
mod expr;
mod node;
mod stmt;
mod type_expr;

pub use expr::*;
pub use node::*;
pub use stmt::*;
pub use type_expr::*;

/// The root AST instance.
///
/// This is the root of the AST, and holds a list of all top-level statements.
/// Instances of this type are created by the parser, and passed to the reducers
/// in the `fluxc_ast_passes` crate, which produce immutable AST instances. The
/// final instance then is sent to the code generator, and turned into valid
/// LLVM code.
#[derive(Debug, PartialEq)]
pub struct AST {
    /// The list of top-level statements in the AST.
    pub stmts: Vec<Node<Stmt>>,
}

impl AST {
    /// Create a new AST instance.
    pub fn new() -> AST {
        AST { stmts: vec![] }
    }
}
