use crate::{ast::Expression, Token};

/// Fills the remaining fields of a struct
#[derive(Debug, Clone)]
pub struct StructBase<'a> {
    /// The dots introducing this base
    pub dots: Token![..],

    /// The expression filling the remaining fields
    pub expression: Box<Expression<'a>>,
}
