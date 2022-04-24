use crate::{Operation, Primitive, Simplify, Type};

/// Represents the union of two types.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    lhs: Box<Type>,
    rhs: Box<Type>,
}

impl Union {
    /// Creates a new union of two types.
    pub fn of(lhs: Type, rhs: Type) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

/// Trait for type union.
pub trait Unify<B> {
    /// Find the union of two types.
    fn unify(&self, b: &B) -> Type;
}

impl Unify<Type> for Type {
    fn unify(&self, b: &Type) -> Type {
        Type::Operation(Operation::Union(Union::of(self.clone(), b.clone()))).simplify()
    }
}

impl Simplify for Union {
    fn simplify(&self) -> Type {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();
        // T | T = T
        if lhs == rhs {
            lhs
        }
        // T | any = any
        else if lhs == Type::Primitive(Primitive::Any) {
            Type::Primitive(Primitive::Any)
        } else if rhs == Type::Primitive(Primitive::Any) {
            Type::Primitive(Primitive::Any)
        }
        // T | never = T
        else if lhs == Type::Primitive(Primitive::Never) {
            rhs
        } else if rhs == Type::Primitive(Primitive::Never) {
            lhs
        } else {
            Type::Operation(Operation::Union(Union::of(lhs.into(), rhs.into())))
        }
    }
}
