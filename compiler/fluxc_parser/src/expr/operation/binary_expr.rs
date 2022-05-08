use fluxc_ast::{BinaryExpr, Node};
use fluxc_errors::CompilerError;
use lazy_static::lazy_static;
use pest::{
    iterators::Pair,
    prec_climber::{Assoc, Operator, PrecClimber},
};

use crate::{Context, Parse, Rule};

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

impl Parse for BinaryExpr {
    #[tracing::instrument]
    fn parse<'i>(
        input: Pair<'i, Rule>,
        context: &mut Context,
    ) -> Result<Node<Self>, CompilerError> {
        todo!()
    }
}
