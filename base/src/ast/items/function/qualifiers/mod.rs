use crate::{ast::Abi, Token};

mod parse;
mod to_tokens;

/// Qualifiers modifying a function
#[derive(Debug, Clone)]
pub struct FunctionQualifiers<'a> {
    /// Indicates the function is a constant function
    pub r#const: Option<Token![const]>,

    /// Indiciates the function is asynchronous
    pub r#async: Option<Token![async]>,

    /// Indicates the function is unsafe
    pub r#unsafe: Option<Token![unsafe]>,

    /// Indicates the function is an external function with a potentially different ABI
    pub r#extern: Option<(Token![extern], Option<Abi<'a>>)>,
}
