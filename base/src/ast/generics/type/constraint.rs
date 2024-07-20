use crate::ast::{Lifetime, Type};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::GenericType;

/// A constraint on a [`GenericType`]
#[derive(Clone)]
pub enum GenericTypeConstraint {
    /// The constraint is a lifetime
    Lifetime(Lifetime),

    /// The constraint is a trait
    Trait(Type),
}
