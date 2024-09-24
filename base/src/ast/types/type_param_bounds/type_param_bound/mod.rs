use crate::ast::Lifetime;

mod parse;
mod to_tokens;

/// A single binding restricting a type parameter
#[derive(Debug, Clone)]
pub enum TypeParamBound<'a> {
    /// The restriction is on the lifetime
    Lifetime(Lifetime<'a>),
}
