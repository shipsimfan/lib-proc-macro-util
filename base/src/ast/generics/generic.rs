use crate::ast::{GenericLifetime, GenericType};

/// A single generic
#[derive(Clone)]
pub enum Generic {
    /// A generic lifetime
    Lifetime(GenericLifetime),

    /// A generic type
    Type(GenericType),
}
