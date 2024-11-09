use crate::ast::items::Struct;

/// The kind of item that a derive is attached to
#[derive(Debug, Clone)]
pub enum DeriveItemKind<'a> {
    /// The item is a structure
    Struct(Struct<'a>),
}
