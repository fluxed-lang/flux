use crate::{Expr, Operation, Primitive, TypeExpr, Typed, Unify};

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
    fn as_type(&self) -> TypeExpr {
        match self {
            Literal::Int(val) => TypeExpr::Primitive(Primitive::IntLiteral(*val)),
            Literal::Float(val) => TypeExpr::Primitive(Primitive::FloatLiteral(*val)),
            Literal::String(val) => TypeExpr::Primitive(Primitive::StringLiteral((*val).clone())),
            Literal::Char(val) => TypeExpr::Primitive(Primitive::CharLiteral(*val)),
            Literal::Bool(val) => TypeExpr::Primitive(match val {
                true => Primitive::True,
                false => Primitive::False,
            }),
            Literal::Array(items) => TypeExpr::Operation(Operation::Array(
                items
                    .iter()
                    .map(|item| item.as_type())
                    .reduce(|out, ty| out.unify(&ty))
                    .unwrap()
                    .into(),
                Some(items.len()),
            )),
        }
    }
}
