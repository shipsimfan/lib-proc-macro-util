use crate::{
    ast::{Attribute, Item, Visibility},
    tokens::{Brace, Ident, Mod, SemiColon, Unsafe},
};

#[derive(Clone)]
pub struct ItemMod {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#unsafe: Option<Unsafe>,
    pub r#mod: Mod,
    pub ident: Ident,
    pub content: Option<(Brace, Vec<Item>)>,
    pub semi_colon: SemiColon,
}
