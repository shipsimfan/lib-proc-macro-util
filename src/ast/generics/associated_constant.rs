use crate::{
    ast::{AngleBracketGenerics, Expression},
    tokens::{Equals, Ident},
};

#[derive(Clone)]
pub struct AssociatedConstant {
    pub ident: Ident,
    pub generics: Option<AngleBracketGenerics>,
    pub equals: Equals,
    pub value: Expression,
}
