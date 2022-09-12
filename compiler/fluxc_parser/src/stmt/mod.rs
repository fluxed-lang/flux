//! Contains the statement AST data structures.

pub(crate) mod declaration;
pub(crate) mod func_decl;
pub(crate) mod module;

use chumsky::{primitive::choice, Parser};
use fluxc_ast::Stmt;

use self::{declaration::declaration, func_decl::func_decl};
use crate::{node, Parsers};

pub(crate) fn stmt<'a>(parsers: &'a Parsers<'a>) {
    let declaration = declaration(&parsers).map(Stmt::Declaration);
    let func_decl = func_decl(&parsers).map(Stmt::FuncDecl);
    let stmt = choice((declaration, func_decl, &parsers.expr)).map_with_span(node);
    parsers.stmt.define(stmt)
}
