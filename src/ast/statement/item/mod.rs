use crate::tokens::TokenStream;

mod r#const;
mod r#enum;
mod extern_crate;
mod r#fn;
mod foreign_item;
mod foreign_mod;
mod r#impl;
mod item_mod;
mod r#macro;
mod r#static;
mod r#struct;
mod r#trait;
mod trait_alias;
mod r#type;
mod union;
mod r#use;

pub use extern_crate::ItemExternCrate;
pub use foreign_item::{
    ForeignItem, ForeignItemFunction, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
};
pub use foreign_mod::ItemForeignMod;
pub use item_mod::ItemMod;
pub use r#const::ItemConst;
pub use r#enum::ItemEnum;
pub use r#fn::{FunctionArgument, ItemFn, PatternType, Receiver, Signature, Variadic};
pub use r#impl::{ImplItem, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType, ItemImpl};
pub use r#macro::ItemMacro;
pub use r#static::ItemStatic;
pub use r#struct::ItemStruct;
pub use r#trait::{
    ItemTrait, TraitItem, TraitItemConst, TraitItemFn, TraitItemMacro, TraitItemType,
};
pub use r#type::ItemType;
pub use r#use::{ItemUse, UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree};
pub use trait_alias::ItemTraitAlias;
pub use union::ItemUnion;

#[derive(Clone)]
pub enum Item {
    Const(ItemConst),
    Enum(ItemEnum),
    ExternCrate(ItemExternCrate),
    Fn(ItemFn),
    ForeignMod(ItemForeignMod),
    Impl(ItemImpl),
    Macro(ItemMacro),
    Mod(ItemMod),
    Static(ItemStatic),
    Struct(ItemStruct),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Type(ItemType),
    Union(ItemUnion),
    Use(ItemUse),
    Verbatim(TokenStream),
}
