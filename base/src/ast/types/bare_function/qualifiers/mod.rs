use crate::{ast::Abi, Token};

mod parse;
mod to_static;
mod to_tokens;

/// Qualifiers adjusting a function
#[derive(Debug, Clone)]
pub struct FunctionTypeQualifiers<'a> {
    /// Indicates if the function is unsafe
    pub r#unsafe: Option<Token![unsafe]>,

    /// Indicates if the function is an external function
    pub r#extern: Option<(Token![extern], Option<Abi<'a>>)>,
}
