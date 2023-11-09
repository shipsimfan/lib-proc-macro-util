use crate::{
    ast::{Attribute, Block},
    tokens::{Async, Move},
};

#[derive(Clone)]
pub struct ExpressionAsync {
    pub attributes: Vec<Attribute>,
    pub r#async: Async,
    pub capture: Option<Move>,
    pub block: Block,
}
