use fluxc_types::{Intersection, Operation, Primitive, Simplify, Type, Union};
use pretty_assertions::assert_eq;

#[test]
fn simplify_union() {
    // string | string = string
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::String)
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string | any = any
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::Any)
        )))
        .simplify(),
        Type::Primitive(Primitive::Any)
    );
    // string | never = string
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::Never)
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // any | string = any
    assert_eq!(
        Type::Operation(Operation::Union(Union::of(
            Type::Primitive(Primitive::Any),
            Type::Primitive(Primitive::String)
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
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::String)
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string & any = string
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::Any)
        )))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
    // string & never = never
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::String),
            Type::Primitive(Primitive::Never)
        )))
        .simplify(),
        Type::Primitive(Primitive::Never)
    );
    // never & string = never
    assert_eq!(
        Type::Operation(Operation::Intersection(Intersection::of(
            Type::Primitive(Primitive::Never),
            Type::Primitive(Primitive::String)
        )))
        .simplify(),
        Type::Primitive(Primitive::Never)
    );

    // string & (string | number) = string
    assert_eq!(
        Type::Operation(Operation::Intersection(
            Intersection::of(
                Type::Primitive(Primitive::String),
                Type::Operation(Operation::Union(Union::of(
                    Type::Primitive(Primitive::String),
                    Type::Primitive(Primitive::Int)
                )))
            )
        ))
        .simplify(),
        Type::Primitive(Primitive::String)
    );
}
