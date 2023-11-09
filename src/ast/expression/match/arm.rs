use crate::{
    ast::{Attribute, Expression, Pattern},
    tokens::{Comma, FatArrow, If},
};

#[derive(Clone)]
pub struct Arm {
    pub attributes: Vec<Attribute>,
    pub pattern: Pattern,
    pub guard: Option<(If, Box<Expression>)>,
    pub fat_arrow: FatArrow,
    pub body: Box<Expression>,
    pub comma: Option<Comma>,
}
