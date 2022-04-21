use crate::{Operation, Primitive, Type, Typed};

use Operation::*;
use Primitive::Never;

pub trait Intersect<B: Typed> {
    fn intersect(&self, b: &B) -> Type;
}

impl<A: Typed, B: Typed> Intersect<B> for A {
    fn intersect(&self, b: &B) -> Type {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Intersect, Primitive, Type};

    #[test]
    fn test_intersection() {}
}
