use crate::{ast::OuterAttribute, Token};

mod param;

mod parse;
mod to_tokens;

pub use param::{MaybeNamedParam, MaybeNamedParamName};

/// A set of function parameters which might not be named
#[derive(Debug, Clone)]
pub struct MaybeNamedFunctionParameters<'a> {
    /// The first parameter
    pub first: MaybeNamedParam<'a>,

    /// The remaining parameters and their separators
    pub remaining: Vec<(Token![,], MaybeNamedParam<'a>)>,

    /// The attributes and marker for variadic variables if the function has them
    pub variadic: Vec<(Token![,], Vec<OuterAttribute<'a>>, Token![...])>,

    /// A final ending comma
    pub ending: Option<Token![,]>,
}
