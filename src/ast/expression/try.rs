use crate::{
    ast::{Attribute, Expression},
    tokens::Question,
};

#[derive(Clone)]
pub struct ExpressionTry {
    pub attributes: Vec<Attribute>,
    pub expression: Box<Expression>,
    pub question: Question,
}
