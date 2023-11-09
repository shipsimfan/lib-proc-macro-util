use crate::{
    ast::Attribute,
    tokens::{Macro, SemiColon},
};

#[derive(Clone)]
pub struct StatementMacro {
    pub attributes: Vec<Attribute>,
    pub r#macro: Macro,
    pub semi_colon: Option<SemiColon>,
}
