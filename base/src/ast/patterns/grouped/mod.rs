use crate::ast::Pattern;

mod parse;
mod to_static;
mod to_tokens;

/// Enclosing a pattern in parentheses can be used to explicitly control the precedence of compound
/// patterns.
#[derive(Debug, Clone)]
pub struct GroupedPattern<'a> {
    /// The contained pattern
    pub pattern: Box<Pattern<'a>>,
}
