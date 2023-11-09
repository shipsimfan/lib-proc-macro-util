use crate::{
    ast::Expression,
    tokens::{Else, Equals},
};

#[derive(Clone)]
pub struct LocalInit {
    pub equals: Equals,
    pub expression: Box<Expression>,
    pub diverge: Option<(Else, Box<Expression>)>,
}
