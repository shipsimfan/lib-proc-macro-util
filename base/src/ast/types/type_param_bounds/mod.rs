mod type_param_bound;

mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use type_param_bound::TypeParamBound;

use crate::Token;

/// A set of bounds on the types parameters can take
#[derive(Debug, Clone)]
pub struct TypeParamBounds<'a> {
    /// The first required type parameter binding
    pub first: TypeParamBound<'a>,

    /// The remaining parameter bindings with their separators
    pub remaining: Vec<(Token![+], TypeParamBound<'a>)>,

    /// A final trailing separator
    pub end: Option<Token![+]>,
}
