use crate::{ast::ForLifetimes, Token};

mod parameters;
mod qualifiers;
mod return_type;

mod parse;
mod to_static;
mod to_tokens;

pub use parameters::{MaybeNamedFunctionParameters, MaybeNamedParam};
pub use qualifiers::FunctionTypeQualifiers;
pub use return_type::BareFunctionReturnType;

/// A bare function type (i.e. `fn`)
#[derive(Debug, Clone)]
pub struct BareFunctionType<'a> {
    /// The lifetimes the function must be valid for
    pub for_lifetimes: Option<ForLifetimes<'a>>,

    /// Any qualifiers modifying this function
    pub qualifiers: FunctionTypeQualifiers<'a>,

    /// The `fn` token indicating this is a function
    pub r#fn: Token![fn],

    /// The parameters of the function
    pub parameters: Option<MaybeNamedFunctionParameters<'a>>,

    /// The function return type
    pub return_type: Option<BareFunctionReturnType<'a>>,
}
