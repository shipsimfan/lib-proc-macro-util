use crate::{
    ast::GenericArgs,
    tokens::{Identifier, Type},
    Token,
};

pub struct GenericArgsBinding<'a> {
    pub identifier: Identifier,
    pub args: Option<Box<GenericArgs<'a>>>,
    pub equals: Token![=],
    pub r#type: Type,
}
