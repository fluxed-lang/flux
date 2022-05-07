use fluxc_types::{Operation, Primitive, Type, Typed, Unify};

use crate::Expr;

#[derive(Debug, PartialEq)]
/// Enum representing the type of a literal.
pub enum Literal {
    /// An integer literal (e.g. `1234`, `0x1234`, `0o1234`, `0b1001`).
    Int(i64),
    /// A floating-point literal (e.g. `1234.5`, `0x1234.5`, `0o1234.5`,
    /// `0b0110.1`).
    Float(f64),
    /// A string literal (e.g. `"hello"`, `"hello world"`).
    String(String),
    /// A character literal (e.g. `'a'`, `'\n'`).
    Char(char),
    /// A boolean literal (e.g. `true`, `false`).
    Bool(bool),
    /// An array literal (e.g. `[1, 2, 3]`).
    Array(Vec<Box<Expr>>),
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
