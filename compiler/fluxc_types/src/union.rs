use crate::{Operation, Simplify, Type, Typed};

/// Trait for type union.
pub trait Union<B: Typed> {
    /// Find the union of two types.
    fn union(&self, b: &B) -> Type;
}

impl<A: Typed, B: Typed> Union<B> for A {
    fn union(&self, b: &B) -> Type {
        Type::Operation(Operation::Union(self.type_of().into(), b.type_of().into())).simplify()
    }
}
