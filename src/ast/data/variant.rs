use crate::{
    ast::{Attribute, Expression, Fields},
    tokens::{Equals, Ident},
};

#[derive(Clone)]
pub struct Variant {
    pub attributes: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub discriminant: Option<(Equals, Expression)>,
}
