//! Contains the function declaration AST data structures.
//!
//! This module handles:
//! - Local function declarations
//! - External function declarations

use fluxc_ast::{FuncCall, FuncDecl, Node, ParenArgument};
use fluxc_errors::CompilerError;
use pest::iterators::Pair;

use crate::{Context, Parse};

impl Parse for FuncCall {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for ParenArgument {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for FuncDecl {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
