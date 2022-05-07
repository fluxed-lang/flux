use pest::iterators::Pair;

use fluxc_ast::{Node, Stmt};
use fluxc_errors::CompilerError;

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

pub use declaration::*;
pub use func_decl::*;
pub use module::*;

use crate::Parse;

impl Parse for Stmt {
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut crate::Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
