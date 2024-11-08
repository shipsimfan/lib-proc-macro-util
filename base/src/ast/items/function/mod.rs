use crate::{
    ast::{GenericParams, WhereClause},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod abi;
mod body;
mod parameters;
mod qualifiers;
mod return_type;

mod parse;
mod to_tokens;

pub use abi::Abi;
pub use body::FunctionBody;
pub use parameters::{FunctionParam, FunctionParameters, SelfParam};
pub use qualifiers::FunctionQualifiers;
pub use return_type::FunctionReturnType;

/// A function definition
#[derive(Debug, Clone)]
pub struct Function<'a> {
    /// The qualifiers modifying the function
    pub qualifiers: FunctionQualifiers<'a>,

    /// The `fn` token indicating this is a function
    pub r#fn: Token![fn],

    /// The name of the function
    pub name: Cow<'a, Identifier>,

    /// Generic parameters for the function
    pub generic_params: Option<GenericParams<'a>>,

    /// The parameters passed to the function
    pub parameters: Option<FunctionParameters<'a>>,

    /// The function return type
    pub return_type: Option<FunctionReturnType<'a>>,

    /// The where clause restricting the generic parameters
    pub where_clause: Option<WhereClause<'a>>,

    /// The body of the function
    pub body: FunctionBody<'a>,
}
