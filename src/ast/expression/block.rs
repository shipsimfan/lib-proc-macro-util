use crate::ast::{Attribute, Block, Label};

#[derive(Clone)]
pub struct ExpressionBlock {
    pub attributes: Vec<Attribute>,
    pub label: Option<Label>,
    pub block: Block,
}
