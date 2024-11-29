use crate::{
    ast::{GenericArgs, Type},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A generic type which has a value
#[derive(Debug, Clone)]
pub struct GenericArgsBinding<'a> {
    /// The name of the generic argument
    pub identifier: Cow<'a, Identifier>,

    /// Arguments for this argument
    pub args: Option<Box<GenericArgs<'a>>>,

    /// The equals indicating this has a value``
    pub equals: Token![=],

    /// The value
    pub value: Box<Type<'a>>,
}
