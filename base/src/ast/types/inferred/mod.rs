use crate::Token;

mod parse;
mod to_tokens;

/// A type which has been explicitly specified and will be inferred by the compiler
#[derive(Debug, Clone)]
pub struct InferredType(pub Token![_]);
