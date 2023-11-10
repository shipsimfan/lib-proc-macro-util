use crate::{
    ast::Attribute,
    tokens::{Ident, Macro, SemiColon},
};

#[derive(Clone)]
pub struct ItemMacro {
    pub attributes: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub r#macro: Macro,
    pub semi_colon: SemiColon,
}
