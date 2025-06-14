use crate::{
    ast::{Type, TypeParamBounds},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;
mod to_type;

/// A generic type parameter
#[derive(Debug, Clone)]
pub struct TypeParam<'a> {
    /// The name of the generic type
    pub identifier: Cow<'a, Identifier>,

    /// Restrictions on what the type can be
    pub bounds: Option<(Token![:], Option<TypeParamBounds<'a>>)>,

    /// A default type
    pub default: Option<(Token![=], Box<Type<'a>>)>,
}
