use crate::{
    ast::{Attribute, Pattern, Type},
    tokens::Colon,
};

#[derive(Clone)]
pub struct PatternType {
    pub attributes: Vec<Attribute>,
    pub pattern: Box<Pattern>,
    pub colon: Colon,
    pub r#type: Box<Type>,
}
