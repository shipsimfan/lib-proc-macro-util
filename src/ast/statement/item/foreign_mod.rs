use crate::{
    ast::{Attribute, ForeignItem, ABI},
    tokens::{Brace, Unsafe},
};

#[derive(Clone)]
pub struct ItemForeignMod {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Unsafe>,
    pub abi: ABI,
    pub brace: Brace,
    pub items: Vec<ForeignItem>,
}
