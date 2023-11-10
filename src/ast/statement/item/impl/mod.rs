use crate::{
    ast::{Attribute, Generics, Path, Type},
    tokens::{Brace, Default, Exclamation, For, Impl, Unsafe},
};

mod r#const;
mod r#fn;
mod item;
mod r#macro;
mod r#type;

pub use item::ImplItem;
pub use r#const::ImplItemConst;
pub use r#fn::ImplItemFn;
pub use r#macro::ImplItemMacro;
pub use r#type::ImplItemType;

#[derive(Clone)]
pub struct ItemImpl {
    pub attributes: Vec<Attribute>,
    pub default: Option<Default>,
    pub r#unsafe: Option<Unsafe>,
    pub r#impl: Impl,
    pub generics: Generics,
    pub r#trait: Option<(Option<Exclamation>, Path, For)>,
    pub self_type: Box<Type>,
    pub brace: Brace,
    pub items: Vec<ImplItem>,
}
