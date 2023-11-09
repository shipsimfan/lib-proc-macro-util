use crate::ast::{Attribute, Expression};

#[derive(Clone)]
pub struct ExpressionGroup {
    pub attributes: Vec<Attribute>,
    pub group: proc_macro::Group,
    pub expression: Box<Expression>,
}
