use crate::{
    ast::Attribute,
    tokens::{Macro, SemiColon},
};

#[derive(Clone)]
pub struct ImplItemMacro {
    pub attributes: Vec<Attribute>,
    pub r#macro: Macro,
    pub semi_colon: SemiColon,
}
