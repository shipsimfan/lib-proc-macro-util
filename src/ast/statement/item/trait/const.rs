use crate::{
    ast::{Attribute, Expression, Generics, Type},
    tokens::{Colon, Const, Equals, Ident, SemiColon},
};

#[derive(Clone)]
pub struct TraitItemConst {
    pub attributes: Vec<Attribute>,
    pub r#const: Const,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Colon,
    pub r#type: Type,
    pub default: Option<(Equals, Expression)>,
    pub semi_colon: SemiColon,
}
