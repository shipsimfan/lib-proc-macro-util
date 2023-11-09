use crate::{
    ast::{Attribute, Expression, Lifetime},
    tokens::Break,
};

#[derive(Clone)]
pub struct ExpressionBreak {
    pub attributes: Vec<Attribute>,
    pub r#break: Break,
    pub label: Option<Lifetime>,
    pub expression: Option<Box<Expression>>,
}
