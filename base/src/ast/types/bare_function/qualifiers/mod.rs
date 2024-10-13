use crate::{ast::Abi, Token};

#[derive(Debug, Clone)]
pub struct FunctionTypeQualifiers {
    pub r#unsafe: Option<Token![unsafe]>,
    pub r#extern: Option<(Token![extern], Option<Abi>)>,
}
