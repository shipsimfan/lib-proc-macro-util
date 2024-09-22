use crate::{ast::Lifetime, Generator, ToTokens};

impl<'a> ToTokens for Lifetime<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Lifetime::Identifier(quote, identifier) => {
                quote.to_tokens(generator);
                identifier.clone().to_tokens(generator);
            }
            Lifetime::IdentifierOwned(quote, identifier) => {
                quote.to_tokens(generator);
                identifier.to_tokens(generator);
            }
            Lifetime::Underscore(quote, underscore) => {
                quote.to_tokens(generator);
                underscore.to_tokens(generator);
            }
        }
    }
}
