use crate::{
    ast::{Attribute, Block, Expression, Label},
    tokens::While,
};

#[derive(Clone)]
pub struct ExpressionWhile {
    pub attributes: Vec<Attribute>,
    pub label: Option<Label>,
    pub r#while: While,
    pub condition: Box<Expression>,
    pub body: Block,
}
