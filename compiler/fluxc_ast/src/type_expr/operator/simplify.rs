use crate::{Operation, Primitive, TypeExpr};

// Trait for simplifying a type tree.
pub trait Simplify {
    /// Simplify the type tree using basic logical axioms.
    fn simplify(&self) -> TypeExpr;
}

impl Simplify for Operation {
    fn simplify(&self) -> TypeExpr {
        match self {
            Operation::Intersection(intersection) => intersection.simplify(),
            Operation::Union(union) => union.simplify(),
            _ => TypeExpr::Operation(self.clone()),
        }
    }
}

impl Simplify for Primitive {
    fn simplify(&self) -> TypeExpr {
        match self {
            Primitive::Tuple(tuple) => {
                let inner_types: Vec<_> = tuple.iter().map(|ty| ty.simplify()).collect();
                TypeExpr::Primitive(Primitive::Tuple(inner_types))
            }
            _ => TypeExpr::Primitive(self.clone()),
        }
    }
}

impl Simplify for TypeExpr {
    fn simplify(&self) -> TypeExpr {
        match self {
            TypeExpr::Operation(op) => op.simplify(),
            TypeExpr::Primitive(primitive) => primitive.simplify(),
            TypeExpr::Circular(inner) => TypeExpr::Circular(inner.simplify().into()),
            t => t.clone(),
        }
    }
}
