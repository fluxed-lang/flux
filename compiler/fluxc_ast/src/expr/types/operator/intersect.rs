use crate::{Operation, Primitive, Simplify, TypeExpr, Unify, Union};

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub lhs: Box<TypeExpr>,
    pub rhs: Box<TypeExpr>,
}

impl Intersection {
    pub fn of(lhs: TypeExpr, rhs: TypeExpr) -> Self {
        Self { lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
}

/// A trait for type intersection.
pub trait Intersect<B> {
    fn intersect(&self, b: &B) -> TypeExpr;
}

// implement Intersect for Type
impl Intersect<TypeExpr> for TypeExpr {
    fn intersect(&self, b: &TypeExpr) -> TypeExpr {
        Intersection::of(self.clone(), b.clone()).simplify()
    }
}

impl Into<TypeExpr> for Intersection {
    fn into(self) -> TypeExpr {
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
    fn simplify(&self) -> TypeExpr {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();
        // T & T = T
        if lhs == rhs {
            return lhs;
        }
        // T & any = T
        if lhs == TypeExpr::Primitive(Primitive::Any) {
            return rhs;
        }
        if rhs == TypeExpr::Primitive(Primitive::Any) {
            return lhs;
        }
        // T & never = never
        if lhs == TypeExpr::Primitive(Primitive::Never) {
            return TypeExpr::Primitive(Primitive::Never);
        }
        if rhs == TypeExpr::Primitive(Primitive::Never) {
            return TypeExpr::Primitive(Primitive::Never);
        }
        if lhs == rhs {
            return lhs.into();
        }
        // more complex intersection
        match (&lhs, &rhs) {
            // (A | B) & (C | D) = (A & C) | (A & D) | (B & C) | (B & D)
            (
                TypeExpr::Operation(Operation::Union(Union { lhs: a, rhs: b })),
                TypeExpr::Operation(Operation::Union(Union { lhs: c, rhs: d })),
            ) => {
                return a
                    .intersect(&c)
                    .unify(&a.intersect(&d))
                    .unify(&b.intersect(&c))
                    .unify(&b.intersect(&d))
                    .simplify();
            }
            // T & (A | B) = (T & A) | (T & B)
            (t, TypeExpr::Operation(Operation::Union(Union { lhs, rhs }))) => {
                return lhs.intersect(&t).unify(&rhs.intersect(&t)).simplify();
            }
            // A & B where A and B's are primitives
            (TypeExpr::Primitive(a), TypeExpr::Primitive(b)) => {
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

        TypeExpr::Operation(Operation::Intersection(Intersection::of(lhs.into(), rhs.into())))
    }
}
#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{Intersect, Primitive, TypeExpr};

    #[test]
    fn primitive_intersection() {
        // string & string = string
        assert_eq!(
            TypeExpr::Primitive(Primitive::String)
                .intersect(&TypeExpr::Primitive(Primitive::String)),
            TypeExpr::Primitive(Primitive::String)
        );
        // string & any = string
        assert_eq!(
            TypeExpr::Primitive(Primitive::String).intersect(&TypeExpr::Primitive(Primitive::Any)),
            TypeExpr::Primitive(Primitive::String)
        );
        // string & never = never
        assert_eq!(
            TypeExpr::Primitive(Primitive::String)
                .intersect(&TypeExpr::Primitive(Primitive::Never)),
            TypeExpr::Primitive(Primitive::Never)
        );
        // string & "hello" = "hello"
        assert_eq!(
            TypeExpr::Primitive(Primitive::String)
                .intersect(&TypeExpr::Primitive(Primitive::StringLiteral("hello".to_string()))),
            TypeExpr::Primitive(Primitive::StringLiteral("hello".to_string()))
        );
    }
}
