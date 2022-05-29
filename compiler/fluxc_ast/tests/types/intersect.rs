//! Integration tests for the intersection operator.

use fluxc_types::{Intersection, Operation, Primitive, Simplify, Type, Union};

#[test]
fn test_union_intersection() {
    // (string | int) & (int | float) = int
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::Int).into()
            )))
            .into(),
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::Int).into(),
                Type::Primitive(Primitive::Float).into()
            )))
            .into()
        )))
        .simplify(),
        Type::Primitive(Primitive::Int)
    );
}
