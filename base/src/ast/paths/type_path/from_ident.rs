use crate::{
    ast::{TypePath, TypePathSegment},
    tokens::Identifier,
};
use std::borrow::Cow;

impl<'a> TypePath<'a> {
    /// Create a [`TypePath`] from a single `identifier`
    pub fn from_ident<I: Into<Cow<'a, Identifier>>>(identifier: I) -> Self {
        TypePath {
            leading: None,
            first: TypePathSegment::from_ident(identifier),
            remaining: Vec::new(),
        }
    }
}
