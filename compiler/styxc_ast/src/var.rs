use styxc_types::Type;

/// Represents a variable in the AST.
#[derive(Clone)]
pub struct Var {
    /// The type of this variable.
    pub field_type: Type,
    /// Whether this variable is a constant.
    pub constant: bool,
}
