use crate::{Operation, Primitive, Simplify, Type, Unify, Union};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub lhs: Box<Type>,
    pub rhs: Box<Type>,
}

impl Intersection {
    pub fn of(lhs: Type, rhs: Type) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

/// A trait for type intersection.
pub trait Intersect<B> {
    fn intersect(&self, b: &B) -> Type;
}

// implement Intersect for Type
impl Intersect<Type> for Type {
    fn intersect(&self, b: &Type) -> Type {
        Intersection::of(self.clone(), b.clone()).simplify()
    }
}

impl Into<Type> for Intersection {
    fn into(self) -> Type {
        self.simplify()
    }
}

/// Implement Simplify trait for Intersection. Simplification of intersections
/// relies on a few logical rules:
/// - T & T = T
/// - T & any = T
/// - T & never = never
/// - T & (A | B) = (T & A) | (T & B)
/// - (A | B) & (C | D) = (A & C) | (A & D) | (B & C) | (B & D)
/// These are applied in order in an attempt to simplify the tree.
impl Simplify for Intersection {
    fn simplify(&self) -> Type {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();
        // T & T = T
        if lhs == rhs {
            return lhs;
        }
        // T & any = T
        if lhs == Type::Primitive(Primitive::Any) {
            return rhs;
        }
        if rhs == Type::Primitive(Primitive::Any) {
            return lhs;
        }
        // T & never = never
        if lhs == Type::Primitive(Primitive::Never) {
            return Type::Primitive(Primitive::Never);
        }
        if rhs == Type::Primitive(Primitive::Never) {
            return Type::Primitive(Primitive::Never);
        }
        if lhs == rhs {
            return lhs.into();
        }
        // more complex intersection
        match (&lhs, &rhs) {
            // (A | B) & (C | D) = (A & C) | (A & D) | (B & C) | (B & D)
            (
                Type::Operation(Operation::Union(Union { lhs: a, rhs: b })),
                Type::Operation(Operation::Union(Union { lhs: c, rhs: d })),
            ) => {
                return a
                    .intersect(&c)
                    .unify(&a.intersect(&d))
                    .unify(&b.intersect(&c))
                    .unify(&b.intersect(&d))
                    .simplify();
            }
            // T & (A | B) = (T & A) | (T & B)
            (t, Type::Operation(Operation::Union(Union { lhs, rhs }))) => {
                return lhs.intersect(&t).unify(&rhs.intersect(&t)).simplify();
            }
            // A & B where A and B's are primitives
            (Type::Primitive(a), Type::Primitive(b)) => {
                if a == b {
                    return a.into();
                }
                // intersect primitive int literals
                let int = match (&a, &b) {
                    (Primitive::IntLiteral(_), Primitive::Int) => a.into(),
                    (Primitive::Int, Primitive::IntLiteral(_)) => b.into(),
                    _ => Primitive::Never.into(),
                };
                if int != Primitive::Never.into() {
                    return int;
                }
                // intersect primitive float literals
                let float = match (&a, &b) {
                    (Primitive::FloatLiteral(_), Primitive::Float) => a.into(),
                    (Primitive::Float, Primitive::FloatLiteral(_)) => b.into(),
                    _ => Primitive::Never.into(),
                };
                if float != Primitive::Never.into() {
                    return float;
                }
                // intersect primitive string literals
                let string = match (&a, &b) {
                    (Primitive::StringLiteral(_), Primitive::String) => a.into(),
                    (Primitive::String, Primitive::StringLiteral(_)) => b.into(),
                    _ => Primitive::Never.into(),
                };
                if string != Primitive::Never.into() {
                    return string;
                }
            }
            _ => (),
        }

        Type::Operation(Operation::Intersection(Intersection::of(
            lhs.into(),
            rhs.into(),
        )))
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
