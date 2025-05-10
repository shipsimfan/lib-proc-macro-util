use crate::Token;

mod parse;
mod to_tokens;

/// The operators which can compare two expressions
#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    /// Compares if the two expressions are equal
    Equals(Token![==]),

    /// Compares if the two expressions are not equal
    NotEquals(Token![!=]),

    /// Compares if the the left is greater than the right
    GreaterThan(Token![>]),

    /// Compares if the the left is less than the right
    LessThan(Token![<]),

    /// Compares if the the left is greater than or equal to the right
    GreaterThanOrEqual(Token![>=]),

    /// Compares if the the left is less than or equal to the right
    LessThanOrEqual(Token![<=]),
}
