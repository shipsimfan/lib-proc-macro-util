use crate::Token;

mod parse;
mod to_tokens;

/// The arithmetic and logical operators which can apply to two expressions
#[derive(Debug, Clone)]
pub enum ArithmeticOrLogicalOperator {
    /// The two expressions are added
    Add(Token![+]),

    /// The two expressions are subtracted
    Sub(Token![-]),

    /// The two expressions are multiplied
    Mul(Token![*]),

    /// The two expressions are divided
    Div(Token![/]),

    /// The remainder of the division of the two expressions is returned
    Mod(Token![%]),

    /// The two expressions are bit-wise ANDed
    And(Token![&]),

    /// The two expressions are bit-wise ORed
    Or(Token![|]),

    /// The two expressions are bit-wise XORed
    Xor(Token![^]),

    /// The first expression is bit-shifted to the left by the second expression
    Shl(Token![<<]),

    /// The first expression is bit-shifted to the right by the second expression
    Shr(Token![>>]),
}
