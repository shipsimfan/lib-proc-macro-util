use crate::{ast::SimplePathSegment, tokens::Identifier, Token};

impl<'a> From<&'a Identifier> for SimplePathSegment<'a> {
    fn from(identifier: &'a Identifier) -> Self {
        SimplePathSegment::Identifier(identifier)
    }
}

impl<'a> From<Identifier> for SimplePathSegment<'a> {
    fn from(identifier: Identifier) -> Self {
        SimplePathSegment::OwnedIdentifier(identifier)
    }
}

impl<'a> From<&str> for SimplePathSegment<'a> {
    fn from(identifier: &str) -> Self {
        SimplePathSegment::new_identifier(identifier)
    }
}

impl<'a> From<(Token![$], Token![crate])> for SimplePathSegment<'a> {
    fn from(value: (Token![$], Token![crate])) -> Self {
        SimplePathSegment::DollarCrate(value.0, value.1)
    }
}

impl<'a> From<()> for SimplePathSegment<'a> {
    fn from(_: ()) -> Self {
        SimplePathSegment::new_dollar_crate()
    }
}
