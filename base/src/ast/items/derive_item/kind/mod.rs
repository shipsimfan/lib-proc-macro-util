use crate::ast::items::{Enumeration, Struct};

mod parse;
mod to_tokens;

/// The kind of item that a derive is attached to
#[derive(Debug, Clone)]
pub enum DeriveItemKind<'a> {
    /// The item is a structure
    Struct(Struct<'a>),

    /// The item is an enumeration
    Enum(Enumeration<'a>),
}
