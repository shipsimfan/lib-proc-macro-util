use crate::{ast::Attribute, tokens::Literal};

#[derive(Clone)]
pub struct ExpressionLiteral {
    pub attributes: Vec<Attribute>,
    pub literal: Literal,
}
