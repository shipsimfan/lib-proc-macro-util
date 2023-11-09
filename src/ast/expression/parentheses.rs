use crate::{
    ast::{Attribute, Expression},
    tokens::Parenthesis,
};

#[derive(Clone)]
pub struct ExpressionParentheses {
    pub attributes: Vec<Attribute>,
    pub parentheses: Parenthesis,
    pub expression: Box<Expression>,
}
