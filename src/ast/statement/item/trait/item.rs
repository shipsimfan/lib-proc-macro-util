use crate::{
    ast::{TraitItemConst, TraitItemFn, TraitItemMacro, TraitItemType},
    tokens::TokenStream,
};

#[derive(Clone)]
pub enum TraitItem {
    Const(TraitItemConst),
    Fn(TraitItemFn),
    Type(TraitItemType),
    Macro(TraitItemMacro),
    Verbatim(TokenStream),
}
