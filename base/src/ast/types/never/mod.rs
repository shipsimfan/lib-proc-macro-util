use crate::Token;

mod new;
mod parse;
mod to_tokens;

/// A type with no value representing computations that never complete
#[derive(Debug, Clone)]
pub struct NeverType(pub Token![!]);
