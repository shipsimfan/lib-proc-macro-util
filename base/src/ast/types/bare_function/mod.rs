use crate::{ast::ForLifetimes, Token};

mod parameters;
mod qualifiers;
mod return_type;

pub use parameters::{MaybeNamedFunctionParameters, MaybeNamedParam, MaybeNamedParamName};
pub use qualifiers::FunctionTypeQualifiers;
pub use return_type::BareFunctionReturnType;

#[derive(Debug, Clone)]
pub struct BareFunctionType<'a> {
    pub for_lifetimes: Option<ForLifetimes<'a>>,
    pub qualifiers: FunctionTypeQualifiers,
    pub r#fn: Token![fn],
    pub parameters: Option<MaybeNamedFunctionParameters<'a>>,
    pub return_type: Option<BareFunctionReturnType<'a>>,
}
