use crate::{
    ast::{PathIdentSegment, TypePathSegment},
    tokens::Identifier,
};
use std::borrow::Cow;

impl<'a> TypePathSegment<'a> {
    /// Create a [`TypePathSegment`] from a single `identifier`
    pub fn from_ident<I: Into<Cow<'a, Identifier>>>(identifier: I) -> Self {
        TypePathSegment {
            ident: PathIdentSegment::Identifier(identifier.into()),
            generics: None,
        }
    }
}
