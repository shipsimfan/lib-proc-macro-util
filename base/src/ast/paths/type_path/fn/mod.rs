use crate::{ast::TypeNoBounds, Token};

mod inputs;
mod parse;
mod to_tokens;

pub use inputs::TypePathFnInputs;

/// A function in a type path
#[derive(Debug, Clone)]
pub struct TypePathFn<'a> {
    /// The input types to the function
    pub inputs: Option<TypePathFnInputs>,

    /// The return type of the function
    pub r#return: Option<(Token![->], Box<TypeNoBounds<'a>>)>,
}
