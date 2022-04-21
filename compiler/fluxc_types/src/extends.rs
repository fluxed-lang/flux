use crate::{Primitive, Type, Typed};

/// Trait for type extension.
pub trait Extends<Parent: Typed>: Typed {
    /// Returns `true` if this type is a subtype of the given parent type.
    fn extends(&self, b: &Parent) -> Primitive;
}

impl<Child: Typed, Parent: Typed> Extends<Parent> for Child {
    fn extends(&self, parent: &Parent) -> Primitive {
        match (self.type_of(), parent.type_of()) {
            // primitives
            (Type::Primitive(a), Type::Primitive(b)) => {
                match (&a, &b) {
                    // a literal primitive and a non-literal primitive
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
                    // two literal and non-literal primitives
                    // A extends B :- A = B
                    _ => (a == b).into(),
                }
            }
            _ => Primitive::False,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primitive_extends_primitive() {
        use crate::{Extends, Primitive, Type};
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
    }
}
