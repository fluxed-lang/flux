use crate::{Operation, Primitive, Simplify, TypeExpr};

/// Represents the union of two types.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub lhs: Box<TypeExpr>,
    pub rhs: Box<TypeExpr>,
}

impl Union {
    /// Creates a new union of two types.
    pub fn of(lhs: TypeExpr, rhs: TypeExpr) -> Self {
        Self { lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
}

/// Trait for type union.
pub trait Unify<B> {
    /// Find the union of two types.
    fn unify(&self, b: &B) -> TypeExpr;
}

impl Unify<TypeExpr> for TypeExpr {
    fn unify(&self, b: &TypeExpr) -> TypeExpr {
        TypeExpr::Operation(Operation::Union(Union::of(self.clone(), b.clone()))).simplify()
    }
}

impl Into<TypeExpr> for Union {
    fn into(self) -> TypeExpr {
        self.simplify()
    }
}

impl Simplify for Union {
    fn simplify(&self) -> TypeExpr {
        let lhs = self.lhs.simplify();
        let rhs = self.rhs.simplify();
        // T | T = T
        if lhs == rhs {
            return lhs;
        }
        match (&lhs, &rhs) {
            // T | any = any
            (TypeExpr::Primitive(Primitive::Any), _) | (_, TypeExpr::Primitive(Primitive::Any)) => {
                return TypeExpr::Primitive(Primitive::Never)
            }
            // T | never = T
            (TypeExpr::Primitive(Primitive::Never), _)
            | (_, TypeExpr::Primitive(Primitive::Never)) => return lhs,
            // (A | B) | A = A | B
            (TypeExpr::Operation(Operation::Union(Union { lhs: a, rhs: b })), c)
            | (c, TypeExpr::Operation(Operation::Union(Union { lhs: a, rhs: b }))) => {
                if a.as_ref() == c {
                    return TypeExpr::Operation(Operation::Union(Union {
                        lhs: a.clone(),
                        rhs: c.clone().into(),
                    }));
                } else if b.as_ref() == c {
                    return TypeExpr::Operation(Operation::Union(Union {
                        lhs: c.clone().into(),
                        rhs: b.clone(),
                    }));
                }
            }
            _ => {}
        };
        // (A | B) | A = T
        TypeExpr::Operation(Operation::Union(Union::of(lhs.into(), rhs.into())))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{Operation, Primitive, TypeExpr, Unify, Union};

    #[test]
    fn unify_primitives() {
        // string | int = string | int
        assert_eq!(
            TypeExpr::Primitive(Primitive::String).unify(&TypeExpr::Primitive(Primitive::Int)),
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::String),
                TypeExpr::Primitive(Primitive::Int),
            )))
        );
        // string | string = string
        assert_eq!(
            TypeExpr::Primitive(Primitive::String).unify(&TypeExpr::Primitive(Primitive::String)),
            TypeExpr::Primitive(Primitive::String)
        );
    }

    #[test]
    fn unify_unions() {
        // (string | int) | (string | int) = (string | int)
        assert_eq!(
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Operation(Operation::Union(Union::of(
                    TypeExpr::Primitive(Primitive::String),
                    TypeExpr::Primitive(Primitive::Int),
                ))),
                TypeExpr::Operation(Operation::Union(Union::of(
                    TypeExpr::Primitive(Primitive::String),
                    TypeExpr::Primitive(Primitive::Int),
                ))),
            )))
            .unify(&TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::String),
                TypeExpr::Primitive(Primitive::Int),
            )))),
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::String),
                TypeExpr::Primitive(Primitive::Int),
            )))
        );
        // (string | int) | (string | float) = (string | int | float)
        assert_eq!(
            TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Operation(Operation::Union(Union::of(
                    TypeExpr::Primitive(Primitive::String),
                    TypeExpr::Primitive(Primitive::Int),
                ))),
                TypeExpr::Operation(Operation::Union(Union::of(
                    TypeExpr::Primitive(Primitive::String),
                    TypeExpr::Primitive(Primitive::Float),
                ))),
            )))
            .unify(&TypeExpr::Operation(Operation::Union(Union::of(
                TypeExpr::Primitive(Primitive::String),
                TypeExpr::Primitive(Primitive::Int),
            )))),
            TypeExpr::Operation(Operation::Union(Union::of(
                Union::of(
                    TypeExpr::Primitive(Primitive::String),
                    TypeExpr::Primitive(Primitive::Int),
                )
                .into(),
                TypeExpr::Primitive(Primitive::Float),
            )))
        );
    }
}
