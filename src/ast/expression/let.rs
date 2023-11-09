use crate::{
    ast::{Attribute, Expression, Pattern},
    tokens::{Equals, Let},
};

#[derive(Clone)]
pub struct ExpressionLet {
    pub attributes: Vec<Attribute>,
    pub r#let: Let,
    pub pattern: Box<Pattern>,
    pub equals: Equals,
    pub expression: Box<Expression>,
}
