use crate::{ast::Lifetime, tokens::Identifier, Token};

impl<'a> Lifetime<'a> {
    /// Creates a new [`Lifetime`] from `identifier`
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        Lifetime::Identifier(Token!['_](), identifier.into().into())
    }

    /// Creates a new anonymous [`Lifetime`]
    pub fn new_anon() -> Self {
        Lifetime::Underscore(Token!['_](), Token![_]())
    }
}
