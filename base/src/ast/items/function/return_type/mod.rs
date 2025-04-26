use crate::{ast::Type, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A return type from a function
#[derive(Debug, Clone)]
pub struct FunctionReturnType<'a> {
    /// An arrowing indicating the start of the return type
    pub arrow: Token![->],

    /// The return type itself
    pub r#type: Type<'a>,
}
