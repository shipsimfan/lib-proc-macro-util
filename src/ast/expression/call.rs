use crate::{
    ast::{Attribute, Expression, Punctuated},
    tokens::{Comma, Parenthesis},
};

#[derive(Clone)]
pub struct ExpressionCall {
    pub attributes: Vec<Attribute>,
    pub function: Box<Expression>,
    pub parentheses: Parenthesis,
    pub arguments: Punctuated<Expression, Comma>,
}
