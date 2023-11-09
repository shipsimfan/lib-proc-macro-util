use crate::{
    ast::{Attribute, Expression, Punctuated},
    tokens::{Bracket, Comma},
};

#[derive(Clone)]
pub struct ExpressionArray {
    pub attributes: Vec<Attribute>,
    pub bracket: Bracket,
    pub elements: Punctuated<Expression, Comma>,
}
