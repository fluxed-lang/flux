use crate::{Operation, Primitive, Type, Typed};

// Trait for simplifying a type tree.
trait Simplify: Typed {
    /// Simplify the type tree.
    fn simplify(&self) -> Type;
}

impl<S: Typed> Simplify for S {
    fn simplify(&self) -> Type {
        // Rules of simplication:
        // T & T = T
        // T & any = T
        // T & never = never
        // T | T = T
        // T | any = any
        // T | never = T
        match self.type_of() {
            Type::Operation(op) => match op {
                Operation::Intersection(lhs, rhs) => {
                    let lhs = lhs.simplify();
                    let rhs = rhs.simplify();
                    // T & T = T
                    if lhs == rhs {
                        lhs
                    }
                    // T & any = T
                    else if lhs == Type::Primitive(Primitive::Any) {
                        rhs
                    } else if rhs == Type::Primitive(Primitive::Any) {
                        lhs
                    }
                    // T & never = never
                    else if lhs == Type::Primitive(Primitive::Never) {
                        Type::Primitive(Primitive::Never)
                    } else if rhs == Type::Primitive(Primitive::Never) {
						Type::Primitive(Primitive::Never)
                    } else {
                        Type::Operation(Operation::Intersection(lhs.into(), rhs.into()))
                    }
                }
                Operation::Union(lhs, rhs) => {
                    let lhs = lhs.simplify();
                    let rhs = rhs.simplify();
                    // T | T = T
                    if lhs == rhs {
                        lhs
                    }
                    // T | any = any
                    else if lhs == Type::Primitive(Primitive::Any) {
                        Type::Primitive(Primitive::Any)
                    } else if rhs == Type::Primitive(Primitive::Any) {
                        Type::Primitive(Primitive::Any)
                    }
                    // T | never = T
                    else if lhs == Type::Primitive(Primitive::Never) {
                        rhs
                    } else if rhs == Type::Primitive(Primitive::Never) {
                        lhs
                    } else {
                        Type::Operation(Operation::Union(lhs.into(), rhs.into()))
                    }
                }
                _ => Type::Operation(op),
            },
            s => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Operation, Primitive, Type, simplify::Simplify};

    #[test]
    fn simplify_union() {
		// string | string = string
        assert_eq!(
            Type::Operation(Operation::Union(
                Type::Primitive(Primitive::String).into(),
                Type::Primitive(Primitive::String).into()
            )).simplify(),
            Type::Primitive(Primitive::String)
        );
		// string | any = any
		assert_eq!(
			Type::Operation(Operation::Union(
				Type::Primitive(Primitive::String).into(),
				Type::Primitive(Primitive::Any).into()
			)).simplify(),
			Type::Primitive(Primitive::Any)
		);
		// string | never = string
		assert_eq!(
			Type::Operation(Operation::Union(
				Type::Primitive(Primitive::String).into(),
				Type::Primitive(Primitive::Never).into()
			)).simplify(),
			Type::Primitive(Primitive::String)
		);
		// any | string = any
		assert_eq!(
			Type::Operation(Operation::Union(
				Type::Primitive(Primitive::Any).into(),
				Type::Primitive(Primitive::String).into()
			)).simplify(),
			Type::Primitive(Primitive::Any)
		);
    }

	#[test]
	fn simplify_intersection() {
		// string & string = string
		assert_eq!(
			Type::Operation(Operation::Intersection(
				Type::Primitive(Primitive::String).into(),
				Type::Primitive(Primitive::String).into()
			)).simplify(),
			Type::Primitive(Primitive::String)
		);
		// string & any = string
		assert_eq!(
			Type::Operation(Operation::Intersection(
				Type::Primitive(Primitive::String).into(),
				Type::Primitive(Primitive::Any).into()
			)).simplify(),
			Type::Primitive(Primitive::String)
		);
		// string & never = never
		assert_eq!(
			Type::Operation(Operation::Intersection(
				Type::Primitive(Primitive::String).into(),
				Type::Primitive(Primitive::Never).into()
			)).simplify(),
			Type::Primitive(Primitive::Never)
		);
		// never & string = never
		assert_eq!(
			Type::Operation(Operation::Intersection(
				Type::Primitive(Primitive::Never).into(),
				Type::Primitive(Primitive::String).into()
			)).simplify(),
			Type::Primitive(Primitive::Never)
		);
	}
}
