//! Contains definitions for HIR datatypes of Flux functions.

use std::fmt::Debug;

use fluxc_types::{Type, Typed};

/// Trait implemented by all types that represent functions.
///
/// This trait provides utility methods for quickly accessing function
/// information without knowing if it is a class method,
pub trait Function: Debug {
    /// The name of this function.
    fn name(&self) -> String;
    /// The kind of this function.
    fn kind(&self) -> FunctionKind;
    /// The arguments of this function.
    fn args(&self) -> Vec<Argument>;
    /// The return value of this function.
    fn return_type(&self) -> Type;
}

/// Enumeration of function kinds for use in compile-time reflection.
pub enum FunctionKind {
    /// A standard function declaration of the form `x -> y`.
    Orphan,
    /// An inline function declaration of the form `x -> y`, that gets inlined
    /// at compile-time.
    InlineOrphan,
    /// An external function declaration.
    External,
    /// A method declaration inside a class.
    Method,
    /// An inline method declaration.
    InlineMethod,
    /// A method definition inside an interface.
    Abstract,
    /// A default method implementation inside an interface.
    Default,
    /// An inline default method implementation inside an interface.
    InlineDefault,
}

/// An argument to a function definition.
pub struct Argument {
    pub name: String,
    pub ty: Type,
}

impl Typed for dyn Function {
    fn type_of(&self) -> Type {
        todo!()
    }
}
