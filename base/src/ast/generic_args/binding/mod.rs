use crate::{
    ast::GenericArgs,
    tokens::{Identifier, Type},
    Token,
};

mod new;
mod parse;
mod to_tokens;

/// A generic type which has a value
#[derive(Debug, Clone)]
pub struct GenericArgsBinding<'a> {
    /// The name of the generic argument
    pub identifier: Identifier,

    /// Arguments for this argument
    pub args: Option<Box<GenericArgs<'a>>>,

    /// The equals indicating this has a value``
    pub equals: Token![=],

    /// The value
    pub value: Type,
}
