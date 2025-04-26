use crate::ast::Visibility;

mod kind;

mod parse;
mod to_static;
mod to_tokens;

pub use kind::VisItemKind;

/// An item prefixed with a visibility
#[derive(Debug, Clone)]
pub struct VisItem<'a> {
    /// The visibility of the item
    pub visibility: Option<Visibility<'a>>,

    /// The kind of item this is
    pub kind: VisItemKind<'a>,
}
