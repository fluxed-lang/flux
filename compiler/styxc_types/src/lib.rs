use std::error::Error;

#[derive(Debug, Clone)]
pub enum Type {
    /// Represents a 64-bit integer type.
    Int64,
    /// Represents a 32-bit integer type.
    Int32,
    /// Represents a 16-bit integer type.
    Int16,
    /// Represents an 8-bit integer type.
    Int8,
    /// Represents an unsigned 64-bit integer type.
    UInt64,
    /// Represents an unsigned 32-bit integer type.
    UInt32,
    /// Represents an unsigned 16-bit integer type.
    UInt16,
    /// Represents an unsigned 8-bit integer type.
    UInt8,
    /// Represents a 128-bit floating point type.
    Float128,
    /// Represents a 64-bit floating point type.
    Float64,
    /// Represents a boolean type.
    Bool,
    /// Represents a tuple type.
    Tuple(Vec<Type>),
    /// Represents an array type.
    Array(Box<Type>),
    /// Represents a map type.
    Map(Box<Type>, Box<Type>),
    /// Represents a set type.
    Set(Box<Type>),
    /// Represents an optional type.
    Optional(Box<Type>),
    /// Represents a union of types.
    Union(Vec<Type>),
    /// Represents an intersection type.
    Intersection(Vec<Type>),
    /// Represents a type already referred to.
    Circular(Box<Type>),
    /// Represents a type that can never occur.
    Never,
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        equate_types(self, other)
    }
}

impl Type {
    /// Compute the intersection of this type with another.
    pub fn intersect(self, other: Type) -> Type {
        if self == other {
            return self;
        }

        Type::Never
    }
}

/// Test if an intersection type is valid.
pub fn validate_intersection(t: &Type) -> Result<(), Box<dyn Error>> {
    match t {
        Type::Intersection(types) => {
            // iterate over types
            for t in types {
                // test if type is a primitive
                if !is_primitive(t) {
                    continue;
                }
                return Err("Cannot compute intesection type of primitives".into());
            }
            return Ok(());
        }
        _ => Err("Cannot validate intersection type if it isn't an intersection type!".into()),
    }
}

/// Test if one type is included within another. Can be used to test for extension.
pub fn is_subtype(a: &Type, b: &Type) -> bool {
    // set a can never be a member of set a
    if equate_types(a, b) {
        return false;
    };

    match (a, b) {
        (a, Type::Union(types)) => return types.contains(&a),
        _ => false,
    }
}

/// Test if type `a` is equal to type `b`.
pub fn equate_types(a: &Type, b: &Type) -> bool {
    // test if can use primitive equality
    if is_primitive(a) && is_primitive(b) {
        return equate_primitives(a, b);
    };
    match (a, b) {
        _ => false,
    }
}

/// Test if type `t` is a primitive.
pub fn is_primitive(t: &Type) -> bool {
    use Type::*;
    match t {
        Int64 => true,
        Int32 => true,
        Int16 => true,
        Int8 => true,
        UInt64 => true,
        UInt32 => true,
        UInt16 => true,
        UInt8 => true,
        Float128 => true,
        Float64 => true,
        Bool => true,
        _ => false,
    }
}

/// Test if two primitive types are equal.
pub fn equate_primitives(a: &Type, b: &Type) -> bool {
    use Type::*;
    match (a, b) {
        (Int64, Int64) => true,
        (Int32, Int32) => true,
        (Int16, Int16) => true,
        (Int8, Int8) => true,
        (UInt64, UInt64) => true,
        (UInt32, UInt32) => true,
        (UInt16, UInt16) => true,
        (UInt8, UInt8) => true,
        (Float128, Float128) => true,
        (Float64, Float64) => true,
        (Bool, Bool) => true,
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_map_insertion(k: &Type, v: &Type, map: &Type) -> bool {
    use Type::*;
    match map {
        Map(key, value) => is_subtype(key, k) && is_subtype(value, v),
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_set_insertion(v: &Type, set: &Type) -> bool {
    use Type::*;
    match set {
        Set(value) => is_subtype(value, v),
        _ => false,
    }
}

/// Validate if a type is valid for insertion into a map.
pub fn validate_array_insertion(v: &Type, array: &Type) -> bool {
    use Type::*;
    match array {
        Array(value) => is_subtype(value, v),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_equality() {
        assert!(equate_types(&Type::Int64, &Type::Int64));
    }

    #[test]
    fn union_inclusion() {
        assert!(is_subtype(
            &Type::Int64,
            &Type::Union(vec![Type::Int64, Type::Int32])
        ));
    }

    #[test]
    fn test_intersection() {
        let a = Type::Union(vec![Type::Int64, Type::Int32]);
        let b = Type::Union(vec![Type::Int32, Type::Int16]);
        assert!(equate_types(&Type::Int32, &Type::Intersection(vec![a, b])))
    }

    #[test]
    fn test_validate_intersection() {
        let a = Type::Union(vec![Type::Int64, Type::Int32]);
        let b = Type::Union(vec![Type::Int32, Type::Int16]);
        assert!(validate_intersection(&Type::Intersection(vec![a, b])).is_ok())
    }
}
