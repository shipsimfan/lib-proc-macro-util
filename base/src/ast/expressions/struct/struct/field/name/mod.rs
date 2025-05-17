use crate::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod parse;
mod to_static;
mod to_tokens;

/// The name of a field in a struct expression
#[derive(Debug, Clone)]
pub enum StructExprFieldName<'a> {
    /// The field name is an identifier
    Identifier(Cow<'a, Identifier>),

    /// The field name is a tuple
    Tuple(Cow<'a, Literal>),
}
