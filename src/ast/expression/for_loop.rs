use crate::{
    ast::{Attribute, Block, Expression, Label, Pattern},
    tokens::{For, In},
};

#[derive(Clone)]
pub struct ExpressionForLoop {
    pub attributes: Vec<Attribute>,
    pub label: Option<Label>,
    pub r#for: For,
    pub pattern: Box<Pattern>,
    pub r#in: In,
    pub expression: Box<Expression>,
    pub body: Block,
}
