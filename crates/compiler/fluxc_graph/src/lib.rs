pub struct Package {
	name: String,
	version: String,
	dependencies: Vec<Dependency>,
}

impl Package {
	pub fn new(name: String, version: String, dependencies: Vec<Dependency>) -> Self {
		Self {
			name,
			version,
			dependencies,
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn version(&self) -> &str {
		&self.version
	}

	pub fn dependencies(&self) -> &[Dependency] {
		&self.dependencies
	}

	pub fn flatten_to(&self, out: &mut Vec<Package>) {
		for d in self.dependencies {
			d.package.flatten_to(out);
		}
		out.push(self);
	}

	pub fn flatten(self) -> Vec<Package> {
		let mut packages = Vec::new();
		self.flatten_to(&mut packages);
		packages
	}
}

pub struct Dependency {
	package: Package,
	optional: bool,
}

