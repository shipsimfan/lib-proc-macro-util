use crate::ast::Lifetime;

mod constraints;

pub use constraints::GenericLifetimeConstraints;

/// A generic lifetime
#[derive(Clone)]
pub struct GenericLifetime {
    /// The name of the lifetime
    pub lifetime: Lifetime,

    /// The constraints on the lifetime
    pub constraints: Option<GenericLifetimeConstraints>,
}
