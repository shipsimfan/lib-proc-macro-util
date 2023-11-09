use crate::{
    ast::{Attribute, Block},
    tokens::Unsafe,
};

#[derive(Clone)]
pub struct ExpressionUnsafe {
    pub attributes: Vec<Attribute>,
    pub r#unsafe: Unsafe,
    pub block: Block,
}
