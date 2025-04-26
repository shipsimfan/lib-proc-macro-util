use crate::ast::patterns::RangePattern;

mod parse;
mod to_static;
mod to_tokens;

/// A pattern with no top-level alternation
#[derive(Debug, Clone)]
pub enum PatternNoTopAlt<'a> {
    /// A range of values
    Range(RangePattern<'a>),
}
