use crate::{
    ast::{Attribute, Expression},
    tokens::{Await, Dot},
};

#[derive(Clone)]
pub struct ExpressionAwait {
    pub attributes: Vec<Attribute>,
    pub base: Box<Expression>,
    pub dot: Dot,
    pub r#await: Await,
}
