use crate::{
    ast::{GenericArgs, TypeParamBounds},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod parse;
mod to_tokens;

/// Bounds on a generic argument
#[derive(Debug, Clone)]
pub struct GenericArgsBounds<'a> {
    /// The name of the bound
    pub identifier: Cow<'a, Identifier>,

    /// Restrictions on the bound
    pub args: Option<Box<GenericArgs<'a>>>,

    /// Seperator for name a bounds
    pub colon: Token![:],

    /// The bounds themselves
    pub bounds: TypeParamBounds<'a>,
}
