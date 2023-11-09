use crate::{
    ast::{Attribute, Block, Label},
    tokens::Loop,
};

#[derive(Clone)]
pub struct ExpressionLoop {
    pub attributes: Vec<Attribute>,
    pub label: Option<Label>,
    pub r#loop: Loop,
    pub body: Block,
}
