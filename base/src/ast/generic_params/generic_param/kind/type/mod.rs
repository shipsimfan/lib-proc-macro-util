use crate::{
    ast::TypeParamBounds,
    tokens::{Identifier, Type},
    Token,
};
use std::borrow::Cow;

mod parse;
mod to_tokens;

/// A generic type parameter
#[derive(Debug, Clone)]
pub struct TypeParam<'a> {
    /// The name of the generic type
    pub identifier: Cow<'a, Identifier>,

    /// Restrictions on what the type can be
    pub bounds: Option<(Token![:], Option<TypeParamBounds<'a>>)>,

    /// A default type
    pub default: Option<(Token![=], Type)>,
}
