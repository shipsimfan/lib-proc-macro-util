use crate::{
    ast::{Lifetime, LifetimeBounds},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A generic lifetime parameter
#[derive(Debug, Clone)]
pub struct LifetimeParam<'a> {
    /// The lifetime itself
    pub lifetime: Lifetime<'a>,

    /// Restrictions on the lifetime
    pub bounds: Option<(Token![:], LifetimeBounds<'a>)>,
}
