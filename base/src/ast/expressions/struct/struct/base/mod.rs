use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// Fills the remaining fields of a struct
#[derive(Debug, Clone)]
pub struct StructBase<'a> {
    /// The dots introducing this base
    pub dots: Token![..],

    /// The expression filling the remaining fields
    pub expression: Box<Expression<'a>>,
}
