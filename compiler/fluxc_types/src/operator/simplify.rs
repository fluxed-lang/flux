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
