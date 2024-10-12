use crate::ast::types::ParenthesizedType;

mod parse;
mod to_tokens;

/// A type which can be defined without bounds
#[derive(Debug, Clone)]
pub enum TypeNoBounds {
    /// The type is surrounded by parentheses
    Parenthesized(ParenthesizedType),
}
