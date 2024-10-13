use crate::{tokens::Identifier, Token};

#[derive(Debug, Clone)]
pub enum MaybeNamedParamName {
    Identifier(Identifier),
    Underscore(Token![_]),
}
