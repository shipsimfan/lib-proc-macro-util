use crate::{ast::SimplePathSegment, tokens::Identifier, Token};

impl<'a> SimplePathSegment<'a> {
    /// Creates a new [`SimplePathSegment`] from `segment`
    pub fn new<T: Into<SimplePathSegment<'a>>>(segment: T) -> SimplePathSegment<'a> {
        segment.into()
    }

    /// Creates a new [`SimplePathSegment::OwnedIdentifier`] from `identifier`
    pub fn new_identifier(identifier: &str) -> SimplePathSegment<'a> {
        Identifier::new(identifier).into()
    }

    /// Creates a new [`SimplePathSegment::DollarCrate`]
    pub fn new_dollar_crate() -> SimplePathSegment<'a> {
        (Token![$](), Token![crate]()).into()
    }

    /// Creates a new [`SimplePathSegment::Crate`]
    pub fn new_crate() -> SimplePathSegment<'a> {
        Token![crate]().into()
    }

    /// Creates a new [`SimplePathSegment::_Self`]
    pub fn new_self() -> SimplePathSegment<'a> {
        Token![self]().into()
    }

    /// Creates a new [`SimplePathSegment::Super`]
    pub fn new_super() -> SimplePathSegment<'a> {
        Token![super]().into()
    }
}
