use crate::{ast::MaybeIdentifier, Generator, ToTokens};

impl<'a> ToTokens for MaybeIdentifier<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            MaybeIdentifier::Identifier(identifier) => identifier.clone().to_tokens(generator),
            MaybeIdentifier::Underscore(underscore) => underscore.to_tokens(generator),
        }
    }
}
