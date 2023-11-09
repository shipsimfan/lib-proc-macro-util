use crate::{
    ast::{Attribute, Expression},
    tokens::{Brace, Match},
};

mod arm;

pub use arm::Arm;

#[derive(Clone)]
pub struct ExpressionMatch {
    pub attributes: Vec<Attribute>,
    pub r#match: Match,
    pub expression: Box<Expression>,
    pub brace: Brace,
    pub arms: Vec<Arm>,
}
