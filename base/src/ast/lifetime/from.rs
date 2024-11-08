use crate::{ast::Lifetime, tokens::Identifier, Token};

impl<'a> From<(Token!['_], &'a Identifier)> for Lifetime<'a> {
    fn from(value: (Token!['_], &'a Identifier)) -> Self {
        Lifetime::Identifier(value.0, value.1.into())
    }
}

impl<'a> From<&'a Identifier> for Lifetime<'a> {
    fn from(identifier: &'a Identifier) -> Self {
        Lifetime::Identifier(Token!['_](), identifier.into())
    }
}

impl<'a, T: Into<Identifier>> From<(Token!['_], T)> for Lifetime<'a> {
    fn from(value: (Token!['_], T)) -> Self {
        Lifetime::Identifier(value.0, value.1.into().into())
    }
}

impl<'a, T: Into<Identifier>> From<T> for Lifetime<'a> {
    fn from(identifier: T) -> Self {
        Lifetime::new(identifier)
    }
}

impl<'a> From<(Token!['_], Token![_])> for Lifetime<'a> {
    fn from(value: (Token!['_], Token![_])) -> Self {
        Lifetime::Underscore(value.0, value.1)
    }
}

impl<'a> From<Token![_]> for Lifetime<'a> {
    fn from(underscore: Token![_]) -> Self {
        Lifetime::Underscore(Token!['_](), underscore)
    }
}

impl<'a> From<()> for Lifetime<'a> {
    fn from(_: ()) -> Self {
        Lifetime::new_anon()
    }
}
