use crate::{
    ast::{OuterAttribute, Type, Visibility},
    tokens::Identifier,
    Token,
};
use std::borrow::Cow;

mod parse;
mod to_tokens;

/// A named field in a normal structure
#[derive(Debug, Clone)]
pub struct StructField<'a> {
    /// The attributes affecting this field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of this field
    pub visibility: Option<Visibility<'a>>,

    /// The name of this field
    pub name: Cow<'a, Identifier>,

    /// The separator between the name and the type
    pub colon: Token![:],

    /// The type of this field
    pub r#type: Type<'a>,
}
