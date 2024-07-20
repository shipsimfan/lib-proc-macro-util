use crate::{
    ast::{Lifetime, Punctuated},
    Token,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::ast::GenericLifetime;

/// Constraints on a [`GenericLifetime`]
#[derive(Clone)]
pub struct GenericLifetimeConstraints {
    /// The colon indicating the start of the constraints
    pub colon: Token![:],

    /// The constraints on the lifetime
    pub constraints: Punctuated<Lifetime, Token![+]>,
}
