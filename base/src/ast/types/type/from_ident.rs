use crate::{
    ast::{Type, TypePath},
    tokens::Identifier,
};
use std::borrow::Cow;

impl<'a> Type<'a> {
    /// Create a type from a single `identifier`
    pub fn from_ident<I: Into<Cow<'a, Identifier>>>(identifier: I) -> Self {
        Type::Path(TypePath::from_ident(identifier))
    }
}
