use crate::{Operation, Primitive, Type, Union};

/// Trait for type extension.
pub trait Extends<B> {
    /// Returns `true` if this type is a subtype of the given parent type.
    fn extends(&self, b: &B) -> Primitive;
}

impl Extends<Primitive> for Primitive {
    fn extends(&self, b: &Primitive) -> Primitive {
        match (self, b) {
            // primitives
            (Primitive::IntLiteral(_), Primitive::Int)
            | (Primitive::Int, Primitive::IntLiteral(_)) => Primitive::True,
            (Primitive::StringLiteral(_), Primitive::String)
            | (Primitive::String, Primitive::StringLiteral(_)) => Primitive::True,
            (Primitive::True, Primitive::Bool) | (Primitive::Bool, Primitive::True) => {
                Primitive::True
            }
            (Primitive::False, Primitive::Bool) | (Primitive::Bool, Primitive::False) => {
                Primitive::True
            }
            // special cases
            // any
            (_, Primitive::Any) => Primitive::True,
            // never
            (Primitive::Never, Primitive::Never) => Primitive::True,
            (_, Primitive::Never) | (Primitive::Never, _) => Primitive::False,
            // two literal and non-literal primitives
            // A extends B :- A = B
            _ => (self == b).into(),
        }
    }
}

impl Extends<Type> for Type {
    fn extends(&self, parent: &Type) -> Primitive {
        match (self, parent) {
            // primitives
            (Type::Primitive(a), Type::Primitive(b)) => a.extends(b),
            // primitive and an operation
            (Type::Primitive(a), Type::Operation(b)) | (Type::Operation(b), Type::Primitive(a)) => {
                match &b {
                    // A extends B :- A = B
                    Operation::Union(Union { lhs, rhs }) => {
                        if a.extends(&lhs) == Primitive::True {
                            Primitive::True
                        } else if a.extends(&rhs) == Primitive::True {
                            Primitive::True
                        } else {
                            Primitive::False
                        }
                    }
                    _ => Primitive::False,
                }
            }
            // two opereations
            (Type::Operation(a), Type::Operation(b)) => match (&a, &b) {
                _ => (a == b).into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operation, Primitive, Type, Union};

    #[test]
    fn primitive_extends_primitive() {
        use crate::{Primitive, Type};
        // int extends int
        assert_eq!(
            Type::Primitive(Primitive::Int).extends(&Type::Primitive(Primitive::Int)),
            Primitive::True
        );
        // 1 extends int
        assert_eq!(
            Type::Primitive(Primitive::IntLiteral(1)).extends(&Type::Primitive(Primitive::Int)),
            Primitive::True
        );
        // 1 extends 0
        assert_eq!(
            Type::Primitive(Primitive::IntLiteral(1))
                .extends(&Type::Primitive(Primitive::IntLiteral(0))),
            Primitive::False
        );
        // true extends bool
        assert_eq!(
            Type::Primitive(Primitive::True).extends(&Type::Primitive(Primitive::Bool)),
            Primitive::True
        );
        // false extends bool
        assert_eq!(
            Type::Primitive(Primitive::False).extends(&Type::Primitive(Primitive::Bool)),
            Primitive::True
        );
        // string extends string
        assert_eq!(
            Type::Primitive(Primitive::String).extends(&Type::Primitive(Primitive::String)),
            Primitive::True
        );
        // "foo" extends string
        assert_eq!(
            Type::Primitive(Primitive::StringLiteral("foo".to_string()))
                .extends(&Type::Primitive(Primitive::String)),
            Primitive::True
        );
        // "foo" extends "bar"
        assert_eq!(
            Type::Primitive(Primitive::StringLiteral("foo".to_string())).extends(&Type::Primitive(
                Primitive::StringLiteral("bar".to_string())
            )),
            Primitive::False
        );

        // any extends any
        assert_eq!(
            Type::Primitive(Primitive::Any).extends(&Type::Primitive(Primitive::Any)),
            Primitive::True
        );
        // never extends never
        assert_eq!(
            Type::Primitive(Primitive::Never).extends(&Type::Primitive(Primitive::Never)),
            Primitive::True
        );
        // never extends any
        assert_eq!(
            Type::Primitive(Primitive::Never).extends(&Type::Primitive(Primitive::Any)),
            Primitive::True
        );
        // any extends never
        assert_eq!(
            Type::Primitive(Primitive::Any).extends(&Type::Primitive(Primitive::Never)),
            Primitive::False
        );
    }

    #[test]
    fn primitive_extends_union() {
        // int extends int | string
        assert_eq!(
            Type::Primitive(Primitive::Int).extends(&Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::Int).into(),
                Type::Primitive(Primitive::String).into()
            )))),
            Primitive::True
        );
        // int extends string | bool
        assert_eq!(
            Type::Primitive(Primitive::Int).extends(&Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Bool).into()
            )))),
            Primitive::False
        );
    }
}
