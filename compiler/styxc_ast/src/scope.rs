use styxc_types::Type;

use crate::Mutability;

pub struct Variable {
    pub mutability: Mutability,
    pub ty: Type,
}

pub struct Scope {
    pub vars: Vec<Variable>,
}
