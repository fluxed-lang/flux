use fluxc_ast::{Declaration, Expr, FuncDecl, Node, Stmt};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

pub use declaration::*;
pub use func_decl::*;
pub use module::*;

use crate::{Context, Parse, Rule};

impl Parse for Stmt {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::statement);
        let node = context.new_empty(input.as_span());
        // statement rule has only one child
        let iter = input.into_inner();
        let inner = iter.last().unwrap();
        // match rule type
        let stmt = match inner.as_rule() {
            Rule::let_declaration => Stmt::Declaration(Declaration::parse(inner, context)?),
            Rule::func_decl | Rule::extern_func_decl => {
                Stmt::FuncDecl(FuncDecl::parse(inner, context)?)
            }
            Rule::expr => Stmt::Expr(Expr::parse(inner, context)?),
            _ => todo!("{:?}", inner.as_rule()),
        };
        // create node and return
        Ok(node.fill(stmt))
    }
}
