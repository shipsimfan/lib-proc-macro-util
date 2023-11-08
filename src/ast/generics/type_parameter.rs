use crate::{
    ast::{Attribute, Punctuated, Type, TypeParameterBound},
    tokens::{Colon, Equals, Ident, Plus},
};

#[derive(Clone)]
pub struct TypeParameter {
    pub attributes: Vec<Attribute>,
    pub ident: Ident,
    pub colon: Option<Colon>,
    pub bounds: Punctuated<TypeParameterBound, Plus>,
    pub equals: Option<Equals>,
    pub default: Option<Type>,
}
