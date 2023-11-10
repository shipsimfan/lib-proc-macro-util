use crate::{
    ast::{ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType},
    tokens::TokenStream,
};

#[derive(Clone)]
pub enum ImplItem {
    Const(ImplItemConst),
    Fn(ImplItemFn),
    Type(ImplItemType),
    Macro(ImplItemMacro),
    Verbatim(TokenStream),
}
