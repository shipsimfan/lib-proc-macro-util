use crate::{ast::TypeNoBounds, Token};

mod mutability;

mod parse;
mod to_static;
mod to_tokens;

pub use mutability::RawPointerTypeMutability;

/// A raw pointer to another type
#[derive(Debug, Clone)]
pub struct RawPointerType<'a> {
    /// The `*` marking this type as a pointer
    pub asterick: Token![*],

    /// Determines if the value the pointer points to is mutable
    pub mutability: RawPointerTypeMutability,

    /// The type of the value the pointer points to
    pub r#type: Box<TypeNoBounds<'a>>,
}
