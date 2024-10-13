use crate::{ast::TypeNoBounds, Token};

#[derive(Debug, Clone)]
pub struct BareFunctionReturnType<'a> {
    pub right_arrow: Token![->],
    pub r#type: Box<TypeNoBounds<'a>>,
}
