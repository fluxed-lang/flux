//! Contains the function declaration AST data structures.
//!
//! This module handles:
//! - Local function declarations
//! - External function declarations

use pest::iterators::Pair;

use fluxc_ast::{FuncCall, FuncDecl, Node, ParenArgument};
use fluxc_errors::CompilerError;

use crate::{Context, Parse};

impl Parse for FuncCall {
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for ParenArgument {
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}

impl Parse for FuncDecl {
    fn parse<'i>(
        input: Pair<'i, crate::Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
