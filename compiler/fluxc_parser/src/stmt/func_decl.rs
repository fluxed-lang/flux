use fluxc_ast::{Block, FuncCall, FuncDecl, Ident, ParenArgument};
use fluxc_span::Span;
use fluxc_types::Type;
use pest::iterators::Pair;

use crate::{unexpected_rule, Context, PResult, Parse, Rule, util::Contains};

impl Parse for FuncCall {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, crate::Rule>, ctx: &mut Context) -> PResult<Self> {
        todo!()
    }
}

impl Parse for ParenArgument {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, crate::Rule>, ctx: &mut Context) -> PResult<Self> {
        debug_assert_eq!(input.as_rule(), Rule::func_decl_param);
        let node = ctx.new_empty(input.as_span());
        let mut inner = input.into_inner();
        let ident = Ident::parse(inner.next().unwrap(), ctx)?;
        let ty = Type::parse(inner.next().unwrap(), ctx)?;
        Ok(node.fill(ParenArgument { ident, ty }))
    }
}

impl Parse for FuncDecl {
    #[tracing::instrument]
    fn parse<'i>(input: Pair<'i, crate::Rule>, ctx: &mut Context) -> PResult<Self> {
        let span = input.as_span();
        let rule = input.as_rule();
        let mut inner = input.into_inner();
        // parse the identifier
        let ident = Ident::parse(inner.next().unwrap(), ctx)?;
        // if declaration contains parameters, parse them
        let params = if inner.peek().map(|x| x.as_rule()).contains(&Rule::func_decl_params) {
            let params = inner.next().unwrap().into_inner();
            let params: Result<Vec<_>, _> =
                params.map(|param| ParenArgument::parse(param, ctx)).into_iter().collect();
            params?
        } else {
            vec![]
        };
        match rule {
            Rule::func_decl => {
                let ret_ty = if inner.peek().map(|pair| pair.as_rule()).contains(&Rule::type_expr) {
                    Type::parse(inner.next().unwrap(), ctx)?
                } else {
                    ctx.new_node(Span::from_str(ctx.src.clone()).restrict(0..0), Type::Infer(None))
                };
                let body = Block::parse(inner.next().unwrap(), ctx)?;
                Ok(ctx.new_node(span, FuncDecl::Local { ident, params, body, ret_ty }))
            }
            Rule::extern_func_decl => {
                // parse the return type
                let ret_ty = Type::parse(inner.next().unwrap(), ctx)?;
                Ok(ctx.new_node(span, FuncDecl::External { ident, params, ret_ty }))
            }
            rule => unexpected_rule(rule, Rule::func_decl),
        }
    }
}
