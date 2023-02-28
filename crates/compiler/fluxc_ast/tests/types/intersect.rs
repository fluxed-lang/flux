//! Integration tests for the intersection operator.

use fluxc_ast::{Intersection, Operation, Primitive, Simplify, TypeExpr, Union};

#[test]
fn test_union_intersection() {
    // (string | int) & (int | float) = int
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(Intersection::of(
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::String).into(),
                TypeExpr::Primitive(Primitive::Int).into()
            )))
            .into(),
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::Int).into(),
                TypeExpr::Primitive(Primitive::Float).into()
            )))
            .into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::Int)
    );
}
