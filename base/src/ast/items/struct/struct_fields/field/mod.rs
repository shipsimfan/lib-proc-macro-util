use crate::{
    ast::{OuterAttribute, Type, Visibility},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct StructField<'a> {
    pub attributes: Vec<OuterAttribute<'a>>,
    pub visibility: Option<Visibility<'a>>,
    pub name: Cow<'a, Identifier>,
    pub colon: Token![:],
    pub r#type: Type<'a>,
}
