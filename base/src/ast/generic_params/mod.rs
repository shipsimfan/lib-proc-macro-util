use crate::Token;

mod generic_param;
mod parse;
mod to_static;
mod to_tokens;

pub use generic_param::*;

/// A set of generic parameters
#[derive(Debug, Clone)]
pub struct GenericParams<'a> {
    /// The opening of the generic parameters
    pub open: Token![<],

    /// Most of the generic parameters
    pub params: Vec<(GenericParam<'a>, Token![,])>,

    /// The last parameter
    pub last_param: GenericParam<'a>,

    /// The last separator
    pub last_comma: Option<Token![,]>,

    /// The ending of the generic parameters
    pub close: Token![>],
}
