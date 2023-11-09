use crate::{
    ast::{Attribute, Block},
    tokens::Try,
};

#[derive(Clone)]
pub struct ExpressionTryBlock {
    pub attributes: Vec<Attribute>,
    pub r#try: Try,
    pub block: Block,
}
