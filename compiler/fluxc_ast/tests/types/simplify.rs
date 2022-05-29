//! Integration tests for the simplification operator.

use fluxc_types::{Intersection, Operation, Primitive, Simplify, Type, Union};
use pretty_assertions::assert_eq;

#[test]
fn simplify_union() {
    // string | string = string
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::String).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string | any = any
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::Any).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::Any)
    );
    // string | never = string
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::Never).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // any | string = any
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::Any).into(),
            Type::Primitive(Primitive::String).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::Any)
    );
}

#[test]
fn simplify_intersection() {
    // string & string = string
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::String).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string & any = string
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::Any).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string & never = never
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::String).into(),
            Type::Primitive(Primitive::Never).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::Never)
    );
    // never & string = never
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::Never).into(),
            Type::Primitive(Primitive::String).into()
        )))
        .simplify(),
        Type::Primitive(Primitive::Never)
    );

    // string & (string | number) = string
    assert_eq!(
        Type::Operation(Operation::Intersection(
            Intersection::of(
                Type::Primitive(Primitive::String).into(),
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String).into(),
                    Type::Primitive(Primitive::Int).into()
                )))
            )
            .into()
        ))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
}
