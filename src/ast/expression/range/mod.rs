use crate::ast::{Attribute, Expression};

mod limits;

pub use limits::RangeLimits;

#[derive(Clone)]
pub struct ExpressionRange {
    pub attributes: Vec<Attribute>,
    pub start: Option<Box<Expression>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expression>>,
}
