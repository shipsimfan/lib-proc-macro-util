use crate::{ast::Lifetime, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A set of restrictions on a lifetime
#[derive(Debug, Clone)]
pub struct LifetimeBounds<'a> {
    /// The leading elements of the lifetime
    pub leading: Vec<(Lifetime<'a>, Token![+])>,

    /// The last lifetime
    pub ending: Option<Lifetime<'a>>,
}
