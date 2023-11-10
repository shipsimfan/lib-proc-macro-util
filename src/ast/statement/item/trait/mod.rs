use crate::{
    ast::{Attribute, Generics, Punctuated, TypeParameterBound, Visibility},
    tokens::{Auto, Brace, Colon, Ident, Plus, Trait, Unsafe},
};

mod r#const;
mod r#fn;
mod item;
mod r#macro;
mod r#type;

pub use item::TraitItem;
pub use r#const::TraitItemConst;
pub use r#fn::TraitItemFn;
pub use r#macro::TraitItemMacro;
pub use r#type::TraitItemType;

#[derive(Clone)]
pub struct ItemTrait {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub r#unsafe: Option<Unsafe>,
    pub auto: Option<Auto>,
    pub r#trait: Trait,
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Colon>,
    pub super_traits: Punctuated<TypeParameterBound, Plus>,
    pub brace: Brace,
    pub items: Vec<TraitItem>,
}
