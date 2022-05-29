use fluxc_types::{Operation, Primitive, Type, Typed};

use crate::Parse;

/// Root enumeration representing type expressions.
#[derive(Debug)]
pub enum TypeExpr {
    /// A primitive type.
    Primitive(Primitive),
    /// A type operation.
    Operation(Operation),
    /// A type to be inferred from the type tree.
    Infer,
    /// A circular type reference.
    Circular(Box<Type>),
}

impl Typed for TypeExpr {
    fn as_type(&self) -> Type {}
}

impl Parse for TypeExpr {}
