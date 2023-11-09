use crate::{
    ast::{Attribute, Expression},
    tokens::{Bracket, SemiColon},
};

#[derive(Clone)]
pub struct ExpressionRepeat {
    pub attributes: Vec<Attribute>,
    pub bracket: Bracket,
    pub expression: Box<Expression>,
    pub semi_colon: SemiColon,
    pub length: Box<Expression>,
}
