use crate::tokens::Literal;
use std::borrow::Cow;

mod parse;
mod to_tokens;

/// An application binary interface (ABI) a function can have
#[derive(Debug, Clone)]
pub struct Abi<'a>(pub Cow<'a, Literal>);
