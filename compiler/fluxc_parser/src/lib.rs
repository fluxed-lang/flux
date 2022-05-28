//! Defines the parser for Flux code.

use fluxc_ast::{Ident, Node, Stmt, AST};
use fluxc_errors::CompilerError;
use fluxc_span::Span;
use pest::{error::Error, iterators::Pair, Parser};

mod expr;
mod stmt;

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
    pub fn new_node<S: Into<Span>, T>(&mut self, span: S, value: T) -> Node<T> {
        let node = Node::new(self.next_id, span.into(), value);
        self.next_id += 1;
        node
    }
    /// Create an empty node.
    pub fn new_empty<S: Into<Span>>(&mut self, span: S) -> Node<()> {
        self.new_node(span, ())
    }
}

fn map_pest_error(error: Error<Rule>) -> CompilerError {
    panic!("{}", error);
    // TODO: proper error parsing
    // match error.variant {
    //     ErrorVariant::ParsingError { positives, negatives } => todo!("map
    // parsing error"),     ErrorVariant::CustomError { message } =>
    // todo!("map custom error"), }
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

/// Small unknown rule function.
#[inline(always)]
pub fn unexpected_rule(received: Rule, scope: Rule) -> ! {
    panic!("unexpected rule '{:?}' received while parsing rule '{:?}'", received, scope);
}
