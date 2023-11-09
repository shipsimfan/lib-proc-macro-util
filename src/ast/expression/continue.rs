use crate::{
    ast::{Attribute, Lifetime},
    tokens::Continue,
};

#[derive(Clone)]
pub struct ExpressionContinue {
    pub attributes: Vec<Attribute>,
    pub r#continue: Continue,
    pub label: Option<Lifetime>,
}
