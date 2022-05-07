//! Provides implementations of `Typed` for various AST structures.

use fluxc_types::{Intersect, Operation, Primitive, Type, Typed, Unify};

use crate::{control::Conditional, Block, Expr, Literal, Node, Stmt};

impl<T: Typed> Typed for Node<T> {
    fn type_of(&self) -> Type {
        self.value.type_of()
    }
}

impl Typed for Expr {
    fn type_of(&self) -> Type {
        match self {
            Expr::Ident(_) => todo!(),
            Expr::BinaryExpr(node) => node
                .value
                .lhs
                .type_of()
                .intersect(&node.value.rhs.type_of()),
            Expr::Block(node) => node.type_of(),
            Expr::FuncCall(_) => todo!(),
            Expr::Conditional(node) => node.type_of(),
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Literal(node) => node.type_of(),
        }
    }
}

impl Typed for Literal {
    fn type_of(&self) -> Type {
        match self {
            Literal::Int(val) => Type::Primitive(Primitive::IntLiteral(*val)),
            Literal::Float(val) => Type::Primitive(Primitive::FloatLiteral(*val)),
            Literal::String(val) => Type::Primitive(Primitive::StringLiteral((*val).clone())),
            Literal::Char(val) => Type::Primitive(Primitive::CharLiteral(*val)),
            Literal::Bool(val) => Type::Primitive(match val {
                true => Primitive::True,
                false => Primitive::False,
            }),
            Literal::Array(items) => Type::Operation(Operation::Array(
                items
                    .iter()
                    .map(|item| item.type_of())
                    .reduce(|out, ty| out.unify(&ty))
                    .unwrap()
                    .into(),
                Some(items.len()),
            )),
        }
    }
}

impl Typed for Block {
    fn type_of(&self) -> Type {
        match self.stmts.last() {
            Some(stmt) => match &stmt.value {
                Stmt::Expr(node) => node.type_of(),
                _ => Type::Primitive(Primitive::Unit),
            },
            None => Type::Primitive(Primitive::Unit),
        }
    }
}

impl Typed for Conditional {
    fn type_of(&self) -> Type {
        let mut ty = self.if_stmt.1.type_of();
        // create union of all branches
        self.else_ifs
            .iter()
            .for_each(|else_if| ty = ty.unify(&else_if.1.type_of()));
        match &self.else_stmt {
            Some(node) => {
                ty = ty.unify(&node.type_of());
            }
            _ => (),
        }
        ty
    }
}
