use crate::ast::{Lifetime, TraitBound};

mod parse;
mod to_static;
mod to_tokens;

/// A single binding restricting a type parameter
#[derive(Debug, Clone)]
pub enum TypeParamBound<'a> {
    /// The restriction is on the lifetime
    Lifetime(Lifetime<'a>),

    /// The restriction is on the implemented traits
    Trait(Box<TraitBound<'a>>),
}
