use crate::Token;

mod parse;
mod to_tokens;

/// The operators which can compare two boolean expressions
#[derive(Debug, Clone)]
pub enum LazyBooleanOperator {
    /// Checks if at least one expression is true
    Or(Token![||]),

    /// Checks if both expressions are true
    And(Token![&&]),
}
