use crate::{
    ast::{Attribute, Block},
    tokens::Const,
};

#[derive(Clone)]
pub struct ExpressionConstant {
    pub attributes: Vec<Attribute>,
    pub r#const: Const,
    pub block: Block,
}
