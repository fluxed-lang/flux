mod extends;
mod intersect;
mod simplify;
mod union;

// export all types
pub use extends::*;
pub use intersect::*;
pub use simplify::*;
pub use union::*;

use crate::{Operation, Type};

impl Into<Type> for Operation {
	fn into(self) -> Type {
		Type::Operation(self)
	}
}
