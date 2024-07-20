use crate::{tokens::Identifier, Token};

/// A lifetime indicator
#[derive(Clone)]
pub struct Lifetime {
    /// The ' indicating this is a lifetime
    pub apostrophe: Token!['_],

    /// The name of the lifetime
    pub name: Identifier,
}
