use crate::ast::OuterAttribute;

mod kind;

mod parse;
mod to_static;
mod to_tokens;

pub use kind::ItemKind;

/// An item
#[derive(Debug, Clone)]
pub struct Item<'a> {
    /// Attributes modifying this item
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The kind of item this is
    pub kind: ItemKind<'a>,
}
