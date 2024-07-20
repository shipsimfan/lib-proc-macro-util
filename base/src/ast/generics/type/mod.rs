use crate::tokens::Identifier;

mod constraint;
mod constraints;

pub use constraint::GenericTypeConstraint;
pub use constraints::GenericTypeConstraints;

/// A generic type
#[derive(Clone)]
pub struct GenericType {
    /// The placeholder name for the type
    pub name: Identifier,

    /// The constraints on the type
    pub constraints: Option<GenericTypeConstraints>,
}
