use crate::{
    ast::{Attribute, Expression},
    tokens::Yield,
};

#[derive(Clone)]
pub struct ExpressionYield {
    pub attributes: Vec<Attribute>,
    pub r#yield: Yield,
    pub expression: Option<Box<Expression>>,
}
