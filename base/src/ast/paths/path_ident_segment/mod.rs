use crate::{tokens::Identifier, Token};

pub enum PathIdentSegment {
    Identifier(Identifier),
    Super(Token![super]),
    _LowerSelf(Token![self]),
    _UpperSelf(Token![Self]),
    _Crate(Token![crate]),
    DollarCrate(Token![$], Token![crate]),
}
