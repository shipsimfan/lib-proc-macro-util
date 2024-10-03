use crate::{tokens::Type, Token};

mod parse;
mod to_tokens;

/// The input types to a function in a type path
#[derive(Debug, Clone)]
pub struct TypePathFnInputs {
    /// The first input type
    pub first: Type,

    /// The remaining input types and their separators
    pub remaining: Vec<(Token![,], Type)>,

    /// The final optional separator
    pub end: Option<Token![,]>,
}
