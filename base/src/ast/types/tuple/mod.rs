use crate::{ast::Type, Token};

mod parse;
mod to_tokens;

/// An ordered heterogenous list of types
#[derive(Debug, Clone)]
pub struct TupleType<'a> {
    /// The list of types followed by separators
    pub types: Vec<(Type<'a>, Token![,])>,

    /// The final type not followed by a separator
    pub last: Option<Box<Type<'a>>>,
}
