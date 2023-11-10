use crate::tokens::TokenStream;

mod r#fn;
mod r#macro;
mod r#static;
mod r#type;

pub use r#fn::ForeignItemFunction;
pub use r#macro::ForeignItemMacro;
pub use r#static::ForeignItemStatic;
pub use r#type::ForeignItemType;

#[derive(Clone)]
pub enum ForeignItem {
    Function(ForeignItemFunction),
    Static(ForeignItemStatic),
    Type(ForeignItemType),
    Macro(ForeignItemMacro),
    Verbatim(TokenStream),
}
