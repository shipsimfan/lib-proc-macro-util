use crate::{ast::Attribute, tokens::Underscore};

#[derive(Clone)]
pub struct ExpressionInfer {
    pub attributes: Vec<Attribute>,
    pub underscore: Underscore,
}
