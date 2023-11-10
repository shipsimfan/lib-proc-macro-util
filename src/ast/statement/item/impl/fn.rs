use crate::{
    ast::{Attribute, Block, Signature, Visibility},
    tokens::Default,
};

#[derive(Clone)]
pub struct ImplItemFn {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub default: Option<Default>,
    pub signature: Signature,
    pub block: Block,
}
