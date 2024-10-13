use crate::tokens::Literal;

mod parse;
mod to_tokens;

/// An application binary interface (ABI) a function can have
#[derive(Debug, Clone)]
pub struct Abi<'a>(pub &'a Literal);
