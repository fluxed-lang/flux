//! Defines the parser for Flux code.

use fluxc_ast::{Ident, Node, Stmt, AST};
use fluxc_errors::CompilerError;
use lazy_static::lazy_static;
use pest::{
    error::{Error, ErrorVariant},
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser, Span,
};

mod expr;
mod stmt;

lazy_static! {
    /// The precedence climber for parsing binary expressions. Since binary expressions are recursive, and the precedence
    /// of operators cannot easily be inferred, we use the PrecClimber to ensure that the parser grammar will not left recurse.
    /// This has the added benefit of handling operator precedence and associativity properly.
    static ref BIN_EXP_CLIMBER: PrecClimber<Rule> = PrecClimber::new(vec![
        // 15
        Operator::new(Rule::binary_op_assign, Assoc::Left) |
        Operator::new(Rule::binary_op_mul_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_div_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_mod_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_plus_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_minus_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_lshift_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_rshift_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_and_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_or_eq, Assoc::Left) |
        Operator::new(Rule::binary_op_bitwise_xor_eq, Assoc::Left),
        // 14
        Operator::new(Rule::binary_op_logical_or, Assoc::Right),
        // 13
        Operator::new(Rule::binary_op_logical_and, Assoc::Right),
        // 12
        Operator::new(Rule::binary_op_eq, Assoc::Right) |
            Operator::new(Rule::binary_op_ne, Assoc::Right),
        // 11
        Operator::new(Rule::binary_op_lt, Assoc::Right) |
            Operator::new(Rule::binary_op_gt, Assoc::Right) |
            Operator::new(Rule::binary_op_le, Assoc::Right) |
            Operator::new(Rule::binary_op_ge, Assoc::Right),
        // 10
        Operator::new(Rule::binary_op_bitwise_or, Assoc::Right),
        // 9
        Operator::new(Rule::binary_op_bitwise_xor, Assoc::Right),
        // 8
        Operator::new(Rule::binary_op_bitwise_and, Assoc::Right),
        // 7
        Operator::new(Rule::binary_op_lshift, Assoc::Right) |
            Operator::new(Rule::binary_op_rshift, Assoc::Right),
        // 6
        Operator::new(Rule::binary_op_plus, Assoc::Right)
            | Operator::new(Rule::binary_op_minus, Assoc::Right),
        // 5
        Operator::new(Rule::binary_op_mul, Assoc::Right)
            | Operator::new(Rule::binary_op_div, Assoc::Right)
            | Operator::new(Rule::binary_op_mod, Assoc::Right)
    ]);
}

/// Internal moduel to prevent leakage of the `Rule` type to external
/// crates.
mod parser {
    use pest_derive::Parser;

    /// The Pest parser for Flux code.
    #[derive(Parser)]
    #[grammar = "./grammar.pest"]
    pub struct FluxParser {}
}

pub(crate) use parser::*;

/// The parser context.
#[derive(Debug)]
struct Context {
    next_id: usize,
}

impl Default for Context {
    fn default() -> Self {
        Self { next_id: 0 }
    }
}

impl Context {
    /// Create a new node from the given pair.
    pub fn new_node<T>(&mut self, span: Span, value: T) -> Node<T> {
        let node = Node::new(self.next_id, span.into(), value);
        self.next_id += 1;
        node
    }
    /// Create an empty node.
    pub fn new_empty(&mut self, span: Span) -> Node<()> {
        self.new_node(span, ())
    }
}

fn map_pest_error(error: Error<Rule>) -> CompilerError {
    match error.variant {
        ErrorVariant::ParsingError { positives, negatives } => todo!(),
        ErrorVariant::CustomError { message } => todo!(),
    }
}

/// Parse an input string into an instance of the Flux `AST`.
#[tracing::instrument]
pub fn parse(input: &str) -> Result<AST, CompilerError> {
    // create the parser context
    let mut context = Context { next_id: 0 };
    // call the pest parser
    let root =
        FluxParser::parse(Rule::flux, &input).map_err(map_pest_error)?.next().unwrap().into_inner();
    // parse top-level statements
    let stmts: Result<Vec<_>, _> = root.map(|rule| Stmt::parse(rule, &mut context)).collect();
    // create and return stmts
    Ok(AST { stmts: stmts? })
}

/// Trait implemented by AST types that can be parsed from the Pest grammar AST.
trait Parse: Sized {
    /// Parse an input Pair into an instance of this type.
    fn parse<'i>(input: Pair<'i, Rule>, context: &mut Context)
        -> Result<Node<Self>, CompilerError>;
}

impl Parse for Ident {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        Ok(context.new_node(input.as_span(), input.as_str().into()))
    }
}

#[cfg(test)]
mod tests {
    use tracing::Level;
    use tracing_subscriber::fmt::format::{FmtSpan, self};

    use crate::parse;

    #[test]
    fn test_parse_stmt() {

        assert_eq!(parse("let x = 1").unwrap().stmts, vec![]);
    }
}