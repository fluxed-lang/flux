use std::error::Error;

use scope::Scope;
use styxc_lexer::Token;

use crate::expr::Expr;

extern crate styxc_types;

pub mod expr;
pub mod func;
pub mod scope;
pub mod util;
pub mod var;

pub struct Parser {
    source: String,
    tokens: Box<dyn Iterator<Item = Token>>, 
    global_scope: Scope
}

impl Parser {
    /// Create a new AST instance.
    pub fn new(raw_source: &str, tokens: Box<dyn Iterator<Item = Token>>) -> Self {
        Self {
            source: raw_source.into(),
            global_scope: Scope::default(),
            tokens: tokens
        }
    }
    
    /// Attempt to build the AST.
    pub fn build(&mut self) -> Result<(), Box<dyn Error>> {
        // store the top level expressions.
        let expressions: Vec<Expr> = match self.build_expressions() {
            Ok(exprs) => exprs,
            Err(e) => return Err(e.into())
        };

        Ok(())
    }

    fn build_expressions(&mut self) -> Result<Vec<Expr>, Box<dyn Error>> {
        Err("not implemented".into())
    }

    /// Attempt to build a function at the current position in the token stream.
    fn build_function(&mut self) -> Result<Expr, Box<dyn Error>> {
        use styxc_lexer::TokenType::*;
        // fetch the identifier of the function
        let next_token = self.tokens.next();
        if next_token.is_none() {
            return Err("attempted to match function at end of token stream".into());
        }
        // can only build a function if it has an identifier.
        let name = match next_token.unwrap().token_type {
            Ident(n) => n,
            _ => return Err("attempted to match function without an identifier".into())
        };
        // get the opening parameter parenthesis, single identifier, or typ
        let ident_or_parens = self.tokens.next();
        if ident_or_parens.is_none() {
            return Err("attempted to match function at end of token stream".into());
        }

        Err("not implemented".into())
    }
}
