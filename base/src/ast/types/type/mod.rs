use crate::ast::types::ParenthesizedType;

mod parse;
mod to_tokens;

/// An syntax element referencing a type
#[derive(Debug, Clone)]
pub enum Type {
    /// The type is surrounded by parentheses
    Parenthesized(ParenthesizedType),
}
