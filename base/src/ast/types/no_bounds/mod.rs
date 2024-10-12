use crate::ast::types::{ImplTraitTypeOneBound, ParenthesizedType};

mod parse;
mod to_tokens;

/// A type which can be defined without bounds
#[derive(Debug, Clone)]
pub enum TypeNoBounds<'a> {
    /// The type is surrounded by parentheses
    Parenthesized(ParenthesizedType<'a>),

    /// An unnamed type implementing one trait
    ImplTraitOneBound(ImplTraitTypeOneBound<'a>),
}
