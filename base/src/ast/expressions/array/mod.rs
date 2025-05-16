mod elements;

mod parse;
mod to_static;
mod to_tokens;

pub use elements::ArrayElements;

/// A grouped expression wraps a single expression, evaluating to that expression
#[derive(Debug, Clone)]
pub struct ArrayExpression<'a> {
    /// The elements which make up
    pub elements: Option<ArrayElements<'a>>,
}
