#[derive(Debug, PartialEq)]
enum Primitive {
    Uint128,
    Uint64,
    Uint32,
    Uint16,
    Uint8,
    Int128,
    Int64,
    Int32,
    Int16,
    Int8,
    Float128,
    Float64,
    Float32,
    Bool,
    String,
    Char,
    Unit,
}

#[derive(Debug, PartialEq)]
enum TypeKind {
    Primitive(Primitive),
    Intersection(Box<TypeKind>, Box<TypeKind>),
}

#[derive(Debug, PartialEq)]
struct Type {
    kind: TypeKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_kind() {
        let t = Type {
            kind: TypeKind::Primitive(Primitive::Uint128),
        };
        assert_eq!(t.kind, TypeKind::Primitive(Primitive::Uint128));
    }
}
