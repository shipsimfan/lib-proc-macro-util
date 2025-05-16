use crate::Token;

mod parse;
mod to_tokens;

/// The operators which can negate an expression
#[derive(Debug, Clone)]
pub enum NegationOperator {
    /// Negates a number
    Numeric(Token![-]),

    /// Negates a boolean
    Boolean(Token![!]),
}
