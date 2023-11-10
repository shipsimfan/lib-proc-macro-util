use crate::{ast::Expression, tokens::SemiColon};

mod item;
mod local;
mod r#macro;

pub use item::{
    ForeignItem, ForeignItemFunction, ForeignItemMacro, ForeignItemStatic, ForeignItemType,
    FunctionArgument, ImplItem, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType, Item,
    ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro, ItemMod,
    ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, PatternType,
    Receiver, Signature, TraitItem, TraitItemConst, TraitItemFn, TraitItemMacro, TraitItemType,
    UseGlob, UseGroup, UseName, UsePath, UseRename, UseTree, Variadic,
};
pub use local::{Local, LocalInit};
pub use r#macro::StatementMacro;

#[derive(Clone)]
pub enum Statement {
    Local(Local),
    Item(Item),
    Expression(Expression, Option<SemiColon>),
    Macro(StatementMacro),
}
