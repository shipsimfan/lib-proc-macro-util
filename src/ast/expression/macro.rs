use crate::{ast::Attribute, tokens::Macro};

#[derive(Clone)]
pub struct ExpressionMacro {
    pub attributes: Vec<Attribute>,
    pub r#macro: Macro,
}
