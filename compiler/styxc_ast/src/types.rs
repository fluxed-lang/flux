use log::trace;
use styxc_types::{Type, equate_types};

use crate::{BinOp, Expr, Literal, LiteralKind};

/// Compute the type of an expression.
pub fn get_expr_type(expr: &Expr) -> Type {
	trace!("get expression type: {:?}", expr);
	use Expr::*;
	match expr {
		Literal(literal) => get_literal_type(literal),
		Ident(ident) => self.find(ident).map_or(Type::Never, |var| var.ty.clone()),
		BinOp(bin_op) => get_bin_op_type(bin_op),
		// TODO: Blocks returning things
		Block(_) => Type::Unit,
	}
}

/// Fetch the type of a literal.
pub fn get_literal_type(literal: &Literal) -> Type {
	trace!("get literal type: {:?}", literal);
	use LiteralKind::*;
	match literal.kind {
		Int(_) => Type::Int,
		Float(_) => Type::Float,
		String(_) => Type::String,
		Char(_) => Type::Char,
		Bool(_) => Type::Bool,
	}
}

/// Fetch the type of a binary operation.
pub fn get_bin_op_type(bin_op: &BinOp) -> Type {
	trace!("get bin op type: {:?}", bin_op);
	let lhs = get_expr_type(&bin_op.lhs);
	let rhs = get_expr_type(&bin_op.rhs);
	if !equate_types(&lhs, &rhs) {
		todo!("bad bin op - error recovery TODO");
	}
	// match comparisons
	use crate::BinOpKind::*;
	match bin_op.kind {
		Eq | Ne | Lt | Gt | Le | Ge => Type::Bool,
		_ => lhs,
	}
}
