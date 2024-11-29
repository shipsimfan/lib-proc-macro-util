use crate::{ast::TypeNoBounds, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A return type from a bare function
#[derive(Debug, Clone)]
pub struct BareFunctionReturnType<'a> {
    /// The right arrow marking the return type
    pub right_arrow: Token![->],

    /// The return type itself
    pub r#type: Box<TypeNoBounds<'a>>,
}
