use crate::ast::patterns::LiteralPattern;

mod parse;
mod to_static;
mod to_tokens;

/// A pattern without a range
#[derive(Debug, Clone)]
pub enum PatternWithoutRange<'a> {
    /// A literal value
    Literal(LiteralPattern<'a>),
}
