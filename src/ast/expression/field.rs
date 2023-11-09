use crate::{
    ast::{Attribute, Expression, Member},
    tokens::Dot,
};

#[derive(Clone)]
pub struct ExpressionField {
    pub attributes: Vec<Attribute>,
    pub base: Box<Expression>,
    pub dot: Dot,
    pub member: Member,
}
