use crate::{ast::Type, Token};

#[derive(Debug, Clone)]
pub struct FunctionReturnType<'a> {
    pub arrow: Token![->],
    pub r#type: Type<'a>,
}
