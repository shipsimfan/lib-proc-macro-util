use crate::{tokens::Identifier, Token};

mod parse;
mod to_tokens;

/// A segment of different types of paths
#[derive(Debug, Clone)]
pub enum PathIdentSegment {
    /// The segment is an identifier
    Identifier(Identifier),

    /// The segment is `super`
    Super(Token![super]),

    /// The segment is `self`
    _LowerSelf(Token![self]),

    /// The segment is `Self`
    _UpperSelf(Token![Self]),

    /// The segment is `crate`
    _Crate(Token![crate]),

    /// The segment is `$crate`
    DollarCrate(Token![$], Token![crate]),
}