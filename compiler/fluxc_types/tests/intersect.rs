use fluxc_types::{Intersection, Operation, Primitive, Simplify, Type, Union};

#[test]
fn test_union_intersection() {
    // (string | int) & (int | float) = int
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::String),
                Type::Primitive(Primitive::Int)
            ))),
            Type::Operation(Operation::Union(Union::of(
                Type::Primitive(Primitive::Int),
                Type::Primitive(Primitive::Float)
            )))
        )))
        .simplify(),
        Type::Primitive(Primitive::Int)
    );
}
