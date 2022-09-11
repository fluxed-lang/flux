//! Contains the statement AST data structures.

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

use chumsky::{prelude::Simple, primitive::choice, recursive::recursive, Parser};
use fluxc_ast::{Node, Stmt};
use fluxc_lexer::Token;

use self::{declaration::declaration, func_decl::func_decl};
use crate::node;

pub(crate) fn stmt() -> impl Parser<Token, Node<Stmt>, Error = Simple<Token>> + Clone {
    recursive(|raw_stmt| {
        let declaration = declaration().map(|decl| Stmt::Declaration(decl));
        let func_decl = func_decl().map(|decl| Stmt::FuncDecl(decl));
        choice((declaration, func_decl)).map_with_span(node)
    })
}
