use crate::ast::Type;

mod parse;
mod to_tokens;

/// A type surrounded by parentheses
#[derive(Debug, Clone)]
pub struct ParenthesizedType<'a> {
    /// The contained type
    pub r#type: Box<Type<'a>>,
}
