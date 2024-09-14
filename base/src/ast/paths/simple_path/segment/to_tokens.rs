use crate::{ast::SimplePathSegment, Generator, ToTokens};

impl<'a> ToTokens for SimplePathSegment<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            SimplePathSegment::Identifier(identifier) => identifier.clone().to_tokens(generator),
            SimplePathSegment::OwnedIdentifier(identifier) => identifier.to_tokens(generator),
            Self::DollarCrate(dollar, krate) => {
                dollar.to_tokens(generator);
                krate.to_tokens(generator);
            }
        }
    }
}
