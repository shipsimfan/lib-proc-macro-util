use crate::{tokens::Identifier, Token};

mod parse;
mod to_tokens;

/// A name a parameter can have which can be anonymous
#[derive(Debug, Clone)]
pub enum MaybeNamedParamName {
    /// The parameter is named
    Identifier(Identifier),

    /// The parameter is anonymous
    Underscore(Token![_]),
}
