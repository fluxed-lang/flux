use crate::{Intersect, Operation, Primitive, Type, Typed, Union};

// Trait for simplifying a type tree.
pub trait Simplify: Typed {
    /// Simplify the type tree.
    fn simplify(&self) -> Type;
}

impl<S: Typed> Simplify for S {
    fn simplify(&self) -> Type {
        // Rules of simplication:
        // T & T = T
        // T & any = T
        // T & never = never
        // T & (A | B) = (T & A) | (T & B)
        // (A | B) & (C | D) = (A & C) | (A & D) | (B & C) | (B & D)
        // T | T = T
        // T | any = any
        // T | never = T
        match self.type_of() {
            Type::Operation(op) => match op {
                Operation::Intersection(lhs, rhs) => {
                    let lhs = lhs.simplify();
                    let rhs = rhs.simplify();
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

                    match (&lhs, &rhs) {
                        // (A | B) & (C | D) = (A & C) | (A & D) | (B & C) | (B & D)
                        (
                            Type::Operation(Operation::Union(a, b)),
                            Type::Operation(Operation::Union(c, d)),
                        ) => {
                            return a
                                .intersect(&c)
                                .union(&a.intersect(&d))
                                .union(&b.intersect(&c))
                                .union(&b.intersect(&d))
                                .simplify();
                        }
                        // T & (A | B) = (T & A) | (T & B)
                        (t, Type::Operation(Operation::Union(a, b))) => {
                            return a.intersect(&t).union(&b.intersect(&t)).simplify();
                        }
                        _ => (),
                    }

                    Type::Operation(Operation::Intersection(lhs.into(), rhs.into()))
                }
                Operation::Union(lhs, rhs) => {
                    let lhs = lhs.simplify();
                    let rhs = rhs.simplify();
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
                        Type::Operation(Operation::Union(lhs.into(), rhs.into()))
                    }
                }
                _ => Type::Operation(op),
            },
            s => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{simplify::Simplify, Operation, Primitive, Type};

    #[test]
    fn simplify_union() {
        // string | string = string
        assert_eq!(
            Type::Operation(Operation::Union(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::String).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string | any = any
        assert_eq!(
            Type::Operation(Operation::Union(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Any).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::Any)
        );
        // string | never = string
        assert_eq!(
            Type::Operation(Operation::Union(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Never).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // any | string = any
        assert_eq!(
            Type::Operation(Operation::Union(
                Type::Primitive(Primitive::Any).into(),
                Type::Primitive(Primitive::String).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::Any)
        );
    }

    #[test]
    fn simplify_intersection() {
        // string & string = string
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::String).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string & any = string
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Any).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string & never = never
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Never).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::Never)
        );
        // never & string = never
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Type::Primitive(Primitive::Never).into(),
                Type::Primitive(Primitive::String).into()
            ))
            .simplify(),
            Type::Primitive(Primitive::Never)
        );
    }
}
