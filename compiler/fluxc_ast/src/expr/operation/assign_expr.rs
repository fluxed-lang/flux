use crate::{Node, Ident, Expr};

#[derive(Debug, PartialEq)]
pub enum AssignmentKind {
    /// The assignment operator, `=`.
    Assign,
    /// The bitwise left-shift assignment operator, `<<=`.
    ShlAssign,
    /// The bitwise right-shift assignment operator, `>>=`.
    ShrAssign,
    /// The bitwise AND assignment operator, `&=`.
    AndAssign,
    /// The bitwise OR assignment operator, `|=`.
    OrAssign,
    /// The bitwise XOR assignment operator, `^=`.
    XorAssign,
    /// The assignment by sum operator, `+=`.
    AddAssign,
    /// The assignment by difference operator, `-=`.
    SubAssign,
    /// The assignment by product operator, `*=`.
    MulAssign,
    /// The assignment by division operator, `/=`.
    DivAssign,
    /// The assignment by modulo operator, `%=`.
    ModAssign,
}

///  A variable assignment.
#[derive(Debug, PartialEq)]

pub struct Assignment {
    /// The identifier being assigned to.
    pub ident: Node<Ident>,
    /// The declared value.
    pub value: Node<Expr>,
    /// The kind of assignment.
    pub kind: AssignmentKind,
}
