use crate::{
    ast::{Attribute, Block, Expression},
    tokens::{Else, If},
};

#[derive(Clone)]
pub struct ExpressionIf {
    pub attributes: Vec<Attribute>,
    pub r#if: If,
    pub condition: Box<Expression>,
    pub then: Block,
    pub r#else: Option<(Else, Box<Expression>)>,
}
