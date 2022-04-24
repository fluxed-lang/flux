use crate::{Operation, Type};

// Trait for simplifying a type tree.
pub trait Simplify {
    /// Simplify the type tree using basic logical axioms.
    fn simplify(&self) -> Type;
}

impl Simplify for Operation {
    fn simplify(&self) -> Type {
        match self {
            Operation::Intersection(intersection) => intersection.simplify(),
            Operation::Union(union) => union.simplify(),
            _ => Type::Operation(self.clone()),
        }
    }
}

impl Simplify for Type {
    fn simplify(&self) -> Type {
        match self {
            Type::Operation(op) => op.simplify(),
            s => s.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Intersection, Operation, Primitive, Simplify, Type, Union};

    #[test]
    fn simplify_union() {
        // string | string = string
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::String).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string | any = any
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Any).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::Any)
        );
        // string | never = string
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Never).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // any | string = any
        assert_eq!(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::Any).into(),
                Type::Primitive(Primitive::String).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::Any)
        );
    }

    #[test]
    fn simplify_intersection() {
        // string & string = string
        assert_eq!(
            Type::Operation(Operation::Intersection(Intersection::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::String).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string & any = string
        assert_eq!(
            Type::Operation(Operation::Intersection(Intersection::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Any).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
        // string & never = never
        assert_eq!(
            Type::Operation(Operation::Intersection(Intersection::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Never).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::Never)
        );
        // never & string = never
        assert_eq!(
            Type::Operation(Operation::Intersection(Intersection::of(
                Type::Primitive(Primitive::Never).into(),
                Type::Primitive(Primitive::String).into()
            )))
            .simplify(),
            Type::Primitive(Primitive::Never)
        );

        // string & (string | number) = string
        assert_eq!(
            Type::Operation(Operation::Intersection(
                Intersection::of(
                    Type::Primitive(Primitive::String).into(),
                    Type::Operation(Operation::Union(Union::of(
                        Type::Primitive(Primitive::String).into(),
                        Type::Primitive(Primitive::Int).into()
                    )))
                )
                .into()
            ))
            .simplify(),
            Type::Primitive(Primitive::String)
        );
    }
}
