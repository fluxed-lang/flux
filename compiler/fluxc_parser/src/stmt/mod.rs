use pest::iterators::Pair;

use fluxc_ast::{Declaration, Node, Stmt};
use fluxc_errors::CompilerError;

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

use crate::{Context, Parse, Rule};

pub use declaration::*;
pub use func_decl::*;
pub use module::*;

impl Parse for Stmt {
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        debug_assert_eq!(input.as_rule(), Rule::statement);
        let node = context.new_empty(input.as_span());
        // statement rule has only one child
        let iter = input.into_inner();
        let inner = iter.last().unwrap();
        // match rule type
        let stmt = match inner.as_rule() {
            Rule::let_declaration => Stmt::Declaration(Declaration::parse(inner, context)?),
            _ => unreachable!(),
        };
        // create node and return
        Ok(node.hydrate(stmt))
    }
}
