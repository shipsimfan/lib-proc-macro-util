use crate::{ast::Abi, Token};

#[derive(Debug, Clone)]
pub struct FunctionQualifiers<'a> {
    pub r#const: Option<Token![const]>,
    pub r#async: Option<Token![async]>,
    pub r#unsafe: Option<Token![unsafe]>,
    pub r#extern: Option<(Token![extern], Option<Abi<'a>>)>,
}
