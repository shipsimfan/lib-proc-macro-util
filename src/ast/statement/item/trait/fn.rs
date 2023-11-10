use crate::{
    ast::{Attribute, Block, Signature},
    tokens::SemiColon,
};

#[derive(Clone)]
pub struct TraitItemFn {
    pub attributes: Vec<Attribute>,
    pub signature: Signature,
    pub default: Option<Block>,
    pub semi_colon: Option<SemiColon>,
}
