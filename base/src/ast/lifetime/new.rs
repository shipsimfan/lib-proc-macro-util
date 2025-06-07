use crate::{ast::Lifetime, tokens::Identifier, Spacing, Token};

impl<'a> Lifetime<'a> {
    /// Creates a new [`Lifetime`] from `identifier`
    pub fn new<T: Into<Identifier>>(identifier: T) -> Self {
        let mut comma = Token!['_]();
        comma.final_spacing = Spacing::Joint;
        Lifetime::Identifier(comma, identifier.into().into())
    }

    /// Creates a new anonymous [`Lifetime`]
    pub fn new_anon() -> Self {
        let mut comma = Token!['_]();
        comma.final_spacing = Spacing::Joint;
        Lifetime::Underscore(comma, Token![_]())
    }
}
