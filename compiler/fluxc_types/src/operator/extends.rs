use crate::{Operation, Primitive, Type, Union, Simplify};

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
            (Type::Primitive(a), Type::Primitive(b)) => a.extends(b),
            // primitive and an operation
            (Type::Primitive(a), Type::Operation(b)) | (Type::Operation(b), Type::Primitive(a)) => {
                b.extends(a)
            }
            // two opereations
            (Type::Operation(a), Type::Operation(b)) => a.extends(b),
			
        }
    }
}

impl Extends<Primitive> for Type {
    fn extends(&self, b: &Primitive) -> Primitive {
        match self {
            Type::Primitive(a) => a.extends(b),
            Type::Operation(a) => a.extends(b),
        }
    }
}

impl Extends<Primitive> for Operation {
    fn extends(&self, b: &Primitive) -> Primitive {
        match self {
            Operation::Union(Union { lhs, rhs }) => {
                if lhs.extends(b) == Primitive::True {
                    Primitive::True
                } else if rhs.extends(b) == Primitive::True {
                    Primitive::True
                } else {
                    Primitive::False
                }
            }
            _ => Primitive::False,
        }
    }
}

impl Extends<Operation> for Type {
	fn extends(&self, b: &Operation) -> Primitive {
		match (self, b) {
			(Type::Primitive(primitive), op) => primitive.extends(op),
			(Type::Operation(lhs), rhs) => lhs.extends(rhs),
		}
	}
}

impl Extends<Operation> for Operation {
    fn extends(&self, b: &Operation) -> Primitive {
        match (self, b) {
            (Operation::Union(a), Operation::Union(b)) => a.extends(b),
            _ => Primitive::False,
        }
    }
}

impl Extends<Operation> for Primitive {
	fn extends(&self, b: &Operation) -> Primitive {
		match (self, b.simplify()) {
			(lhs, Type::Primitive(rhs)) => lhs.extends(&rhs),
			(_, Type::Operation(_)) => todo!("primitive extends unsimplifiable operation"),
		} 
	}
}

impl Extends<Union> for Union {
    fn extends(&self, b: &Union) -> Primitive {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{Extends, Operation, Primitive, Type, Union};

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
