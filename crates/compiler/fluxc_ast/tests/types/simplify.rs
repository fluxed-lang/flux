//! Integration tests for the simplification operator.

use fluxc_ast::{Intersection, Operation, Primitive, Simplify, TypeExpr, Union};
use pretty_assertions::assert_eq;

#[test]
fn simplify_union() {
    // string | string = string
    assert_eq!(
        TypeExpr::Operation(Operation::Union(Union::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::String).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::String)
    );
    // string | any = any
    assert_eq!(
        TypeExpr::Operation(Operation::Union(Union::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::Any).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::Any)
    );
    // string | never = string
    assert_eq!(
        TypeExpr::Operation(Operation::Union(Union::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::Never).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::String)
    );
    // any | string = any
    assert_eq!(
        TypeExpr::Operation(Operation::Union(Union::of(
            TypeExpr::Primitive(Primitive::Any).into(),
            TypeExpr::Primitive(Primitive::String).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::Any)
    );
}

#[test]
fn simplify_intersection() {
    // string & string = string
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(Intersection::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::String).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::String)
    );
    // string & any = string
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(Intersection::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::Any).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::String)
    );
    // string & never = never
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(Intersection::of(
            TypeExpr::Primitive(Primitive::String).into(),
            TypeExpr::Primitive(Primitive::Never).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::Never)
    );
    // never & string = never
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(Intersection::of(
            TypeExpr::Primitive(Primitive::Never).into(),
            TypeExpr::Primitive(Primitive::String).into()
        )))
        .simplify(),
        TypeExpr::Primitive(Primitive::Never)
    );

    // string & (string | number) = string
    assert_eq!(
        TypeExpr::Operation(Operation::Intersection(
            Intersection::of(
                TypeExpr::Primitive(Primitive::String).into(),
                TypeExpr::Operation(Operation::Union(Union::of(
                    TypeExpr::Primitive(Primitive::String).into(),
                    TypeExpr::Primitive(Primitive::Int).into()
                )))
            )
            .into()
        ))
        .simplify(),
        TypeExpr::Primitive(Primitive::String)
    );
}
