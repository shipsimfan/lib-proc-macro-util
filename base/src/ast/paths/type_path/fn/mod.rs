use crate::{ast::TypeNoBounds, Token};

mod inputs;
mod parse;
mod to_tokens;

pub use inputs::TypePathFnInputs;

/// A function in a type path
pub struct TypePathFn {
    /// The input types to the function
    pub inputs: Option<TypePathFnInputs>,

    /// The return type of the function
    pub r#return: Option<(Token![->], TypeNoBounds)>,
}
