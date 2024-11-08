use crate::{ast::SimplePathSegment, tokens::Identifier, Token};

impl<'a> From<&'a Identifier> for SimplePathSegment<'a> {
    fn from(identifier: &'a Identifier) -> Self {
        SimplePathSegment::Identifier(identifier.into())
    }
}

impl<'a> From<Identifier> for SimplePathSegment<'a> {
    fn from(identifier: Identifier) -> Self {
        SimplePathSegment::Identifier(identifier.into())
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

impl<'a> From<Token![crate]> for SimplePathSegment<'a> {
    fn from(krate: Token![crate]) -> Self {
        SimplePathSegment::Crate(krate)
    }
}

impl<'a> From<Token![self]> for SimplePathSegment<'a> {
    fn from(_self: Token![self]) -> Self {
        SimplePathSegment::_Self(_self)
    }
}

impl<'a> From<Token![super]> for SimplePathSegment<'a> {
    fn from(_super: Token![super]) -> Self {
        SimplePathSegment::Super(_super)
    }
}
