use std::path::PathBuf;

/// HIR datatype representing a Flux module and its exported symbols.
pub struct Module {
	pub path: PathBuf
}

/// HIR datatype representing an `import` directive.
pub struct Import {}
