use crate::{
    ast::AngleBracketGenerics,
    tokens::{Equals, Ident, Type},
};

#[derive(Clone)]
pub struct AssociatedType {
    pub ident: Ident,
    pub generics: Option<AngleBracketGenerics>,
    pub equals: Equals,
    pub r#type: Type,
}
