use crate::{Operation, Primitive, Type};

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

impl Simplify for Primitive {
    fn simplify(&self) -> Type {
        match self {
            Primitive::Tuple(tuple) => {
                let inner_types: Vec<_> = tuple.iter().map(|ty| ty.simplify()).collect();
                Type::Primitive(Primitive::Tuple(inner_types))
            }
            _ => Type::Primitive(self.clone()),
        }
    }
}

impl Simplify for Type {
    fn simplify(&self) -> Type {
        match self {
            Type::Operation(op) => op.simplify(),
            Type::Primitive(primitive) => primitive.simplify(),
			Type::Circular(inner) => Type::Circular(inner.simplify().into()),
			t => t.clone(),
        }
    }
}
