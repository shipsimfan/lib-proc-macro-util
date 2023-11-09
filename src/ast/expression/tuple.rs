use crate::{
    ast::{Attribute, Expression, Punctuated},
    tokens::{Comma, Parenthesis},
};

#[derive(Clone)]
pub struct ExpressionTuple {
    pub attributes: Vec<Attribute>,
    pub parentheses: Parenthesis,
    pub elements: Punctuated<Expression, Comma>,
}
