//! Provides implementations of `Typed` for various AST structures.

use fluxc_types::{Intersect, Primitive, Type, Typed, Unify};

use crate::{Block, Expr, Literal, Node};

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
            Expr::Conditional(node) => {
                let mut ty = node.value.if_stmt.1.type_of();
                // create union of all branches
                node.value
                    .else_ifs
                    .iter()
                    .for_each(|else_if| ty = ty.unify(&else_if.1.type_of()));
                match node.value.else_stmt {
                    Some(node) => {
                        ty = ty.unify(&node.type_of());
                    }
                    _ => (),
                }
                ty
            }
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
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
            Literal::Array(items) => todo!(),
        }
    }
}

impl Typed for Block {
    fn type_of(&self) -> Type {
        todo!()
    }
}
