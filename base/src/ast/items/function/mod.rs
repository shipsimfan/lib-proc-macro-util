use crate::{
    ast::{GenericParams, WhereClause},
    tokens::Identifier,
    Token,
};

mod abi;
mod body;
mod parameters;
mod qualifiers;
mod return_type;

pub use abi::Abi;
pub use body::FunctionBody;
pub use parameters::{FunctionParam, FunctionParameters, SelfParam};
pub use qualifiers::FunctionQualifiers;
pub use return_type::FunctionReturnType;

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub qualifiers: FunctionQualifiers<'a>,
    pub r#fn: Token![fn],
    pub name: &'a Identifier,
    pub generic_params: Option<GenericParams<'a>>,
    pub parameters: Option<FunctionParameters<'a>>,
    pub return_type: Option<FunctionReturnType<'a>>,
    pub where_clause: Option<WhereClause<'a>>,
    pub body: FunctionBody<'a>,
}
