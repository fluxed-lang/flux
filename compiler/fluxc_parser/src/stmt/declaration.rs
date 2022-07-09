use fluxc_ast::{Declaration, Expr, Ident, Mutability, Node};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse, Rule};

impl Parse for Declaration {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> PResult<Self> {
        let node = context.new_empty(input.as_span());
        // declaration is either a let or a mutable declaration.
        let decl = match input.as_rule() {
            Rule::let_declaration => {
                let mut iter = input.into_inner();
                // parse ident
                let ident = iter.next().unwrap();
                let ident = Ident::parse(ident, context)?;
                // if next token is mut, parse mutable declaration
                let mutability =
                    if iter.peek().map(|r| r.as_rule() == Rule::mut_token).unwrap_or(false) {
                        iter.next().unwrap();
                        Mutability::Mutable
                    } else {
                        Mutability::Immutable
                    };
                // parse value
                let value = iter.next().unwrap();
                let value = Expr::parse(value, context)?;

                Declaration { explicit_ty: None, mutability, value, ident }
            }
            _ => unreachable!(),
        };
        Ok(node.fill(decl))
    }
}
