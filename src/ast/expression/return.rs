use crate::{
    ast::{Attribute, Expression},
    tokens::Return,
};

#[derive(Clone)]
pub struct ExpressionReturn {
    pub attributes: Vec<Attribute>,
    pub r#return: Return,
    pub expression: Option<Box<Expression>>,
}
