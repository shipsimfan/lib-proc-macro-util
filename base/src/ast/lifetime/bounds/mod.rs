use crate::{ast::Lifetime, Token};

/// A set of restrictions on a lifetime
pub struct LifetimeBounds<'a> {
    /// The leading elements of the lifetime
    pub leading: Vec<(Lifetime<'a>, Token![+])>,

    /// The last lifetime
    pub ending: Option<Lifetime<'a>>,
}
