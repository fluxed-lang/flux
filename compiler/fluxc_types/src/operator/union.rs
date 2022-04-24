use crate::{Operation, Primitive, Simplify, Type};

/// Represents the union of two types.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub lhs: Box<Type>,
    pub rhs: Box<Type>,
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

impl Into<Type> for Union {
    fn into(self) -> Type {
        self.simplify()
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{Operation, Primitive, Type, Unify, Union};

    #[test]
    fn unify_primitives() {
        // string | int = string | int
        assert_eq!(
            Type::Primitive(Primitive::String).unify(&Type::Primitive(Primitive::Int)),
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String),
                Type::Primitive(Primitive::Int),
            )))
        );
        // string | string = string
        assert_eq!(
            Type::Primitive(Primitive::String).unify(&Type::Primitive(Primitive::String)),
            Type::Primitive(Primitive::String)
        );
    }

    #[test]
    fn unify_unions() {
        // (string | int) | (string | int) = (string | int)
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Int),
                ))),
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Int),
                ))),
            )))
            .unify(&Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String),
                Type::Primitive(Primitive::Int),
            )))),
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String),
                Type::Primitive(Primitive::Int),
            )))
        );
        // (string | int) | (string | float) = (string | int | float)
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Int),
                ))),
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Float),
                ))),
            )))
            .unify(&Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String),
                Type::Primitive(Primitive::Int),
            )))),
            Type::Operation(Operation::Union(Union::of(
                Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Int),
                )
                .into(),
                Type::Primitive(Primitive::Float),
            )))
        );
    }
}
