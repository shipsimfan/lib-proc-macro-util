use crate::ast::{OuterAttribute, Visibility};

mod kind;
mod parse;
mod to_tokens;

pub use kind::DeriveItemKind;

/// An item which can have a derive macro attached
#[derive(Debug, Clone)]
pub struct DeriveItem<'a> {
    /// Attributes modifying this item
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility of the item
    pub visibility: Option<Visibility<'a>>,

    /// The item of item this is
    pub kind: DeriveItemKind<'a>,
}
