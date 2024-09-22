use crate::{ast::Lifetime, tokens::Identifier, Token};

impl<'a> Lifetime<'a> {
    /// Creates a new [`Lifetime`] from `identifier`
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        Lifetime::IdentifierOwned(Token!['_](), identifier.into())
    }

    /// Creates a new anonymous [`Lifetime`]
    pub fn new_anon() -> Self {
        Lifetime::Underscore(Token!['_](), Token![_]())
    }
}
