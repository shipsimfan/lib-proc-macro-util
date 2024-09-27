use crate::{
    ast::TypeParamBounds,
    tokens::{Identifier, Type},
    Token,
};

pub struct TypeParam<'a> {
    pub identifier: Identifier,
    pub bounds: Option<(Token![:], Option<TypeParamBounds<'a>>)>,
    pub value: Option<(Token![=], Type)>,
}
