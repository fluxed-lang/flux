use crate::{Operation, Primitive, Simplify, Type, Typed, Union};

pub trait Intersect<B: Typed> {
    fn intersect(&self, b: &B) -> Type;
}

impl<A: Typed, B: Typed> Intersect<B> for A {
    fn intersect(&self, b: &B) -> Type {
        match (self.type_of(), b.type_of()) {
            // A & B where A and B's are primitives
            (Type::Primitive(a), Type::Primitive(b)) => {
                if a == b {
                    return a.type_of();
                }
                // intersect primitive int literals
                let int = match (&a, &b) {
                    (Primitive::IntLiteral(_), Primitive::Int) => a.type_of(),
                    (Primitive::Int, Primitive::IntLiteral(_)) => b.type_of(),
                    _ => Primitive::Never.type_of(),
                };
                if int != Primitive::Never.type_of() {
                    return int;
                }
                // intersect primitive float literals
                let float = match (&a, &b) {
                    (Primitive::FloatLiteral(_), Primitive::Float) => a.type_of(),
                    (Primitive::Float, Primitive::FloatLiteral(_)) => b.type_of(),
                    _ => Primitive::Never.type_of(),
                };
                if float != Primitive::Never.type_of() {
                    return float;
                }
                // intersect primitive string literals
                let string = match (&a, &b) {
                    (Primitive::StringLiteral(_), Primitive::String) => a.type_of(),
                    (Primitive::String, Primitive::StringLiteral(_)) => b.type_of(),
                    _ => Primitive::Never.type_of(),
                };
                if string != Primitive::Never.type_of() {
                    return string;
                }
                // default to intersection representation
                Type::Operation(Operation::Intersection(
                    self.type_of().into(),
                    b.type_of().into(),
                ))
                .simplify()
            }
            // default to pure intersection type
            _ => Type::Operation(Operation::Intersection(
                self.type_of().into(),
                b.type_of().into(),
            ))
            .simplify(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Intersect, Operation, Primitive, Simplify, Type};

    #[test]
    fn test_primitive_intersection() {
        // string & string = string
        assert_eq!(
            Type::Primitive(Primitive::String).intersect(&Type::Primitive(Primitive::String)),
            Type::Primitive(Primitive::String)
        );
        // string & any = string
        assert_eq!(
            Type::Primitive(Primitive::String).intersect(&Type::Primitive(Primitive::Any)),
            Type::Primitive(Primitive::String)
        );
        // string & never = never
        assert_eq!(
            Type::Primitive(Primitive::String).intersect(&Type::Primitive(Primitive::Never)),
            Type::Primitive(Primitive::Never)
        );
        // string & "hello" = "hello"
        assert_eq!(
            Type::Primitive(Primitive::String).intersect(&Type::Primitive(
                Primitive::StringLiteral("hello".to_string())
            )),
            Type::Primitive(Primitive::StringLiteral("hello".to_string()))
        );
    }

    #[test]
    fn test_union_intersection() {
        // (string | int) & (int | float) = int
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Type::Operation(Operation::Union(
                    Type::Primitive(Primitive::String).into(),
                    Type::Primitive(Primitive::Int).into()
                ))
                .into(),
                Type::Operation(Operation::Union(
                    Type::Primitive(Primitive::Int).into(),
                    Type::Primitive(Primitive::Float).into()
                ))
                .into()
            ))
            .simplify(),
            Type::Primitive(Primitive::Int)
        );
    }
}
