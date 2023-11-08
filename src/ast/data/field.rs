use crate::{
    ast::{Attribute, Type, Visibility},
    tokens::{Colon, Ident},
};

#[derive(Clone)]
pub struct Field {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Option<Ident>,
    pub colon: Option<Colon>,
    pub r#type: Type,
}
