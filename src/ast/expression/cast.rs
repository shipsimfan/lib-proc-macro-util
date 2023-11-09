use crate::{
    ast::{Attribute, Expression, Type},
    tokens::As,
};

#[derive(Clone)]
pub struct ExpressionCast {
    pub attributes: Vec<Attribute>,
    pub expression: Box<Expression>,
    pub r#as: As,
    pub r#type: Box<Type>,
}
