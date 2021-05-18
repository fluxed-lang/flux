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
