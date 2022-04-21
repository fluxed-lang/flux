use crate::{Type, Typed};

/// Trait for type union.
trait Union<B: Typed> {
    /// Find the union of two types.
    fn union(&self, b: &B) -> Type;
}
