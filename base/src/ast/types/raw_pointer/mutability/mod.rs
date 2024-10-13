use crate::Token;

mod parse;
mod to_tokens;

/// Determines if a raw pointer is mutable or constant
#[derive(Debug, Clone)]
pub enum RawPointerTypeMutability {
    /// The value at the pointer is mutable
    Mut(Token![mut]),

    /// The value at the pointer is constant
    Const(Token![const]),
}
