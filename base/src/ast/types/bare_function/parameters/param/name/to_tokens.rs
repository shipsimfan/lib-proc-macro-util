use crate::{ast::types::MaybeNamedParamName, Generator, ToTokens};

impl ToTokens for MaybeNamedParamName {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            MaybeNamedParamName::Identifier(identifier) => identifier.to_tokens(generator),
            MaybeNamedParamName::Underscore(underscore) => underscore.to_tokens(generator),
        }
    }
}
