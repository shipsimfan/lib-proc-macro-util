use crate::{tokens::Punctuation, Generator, ToTokens};

impl ToTokens for Punctuation {
    fn to_tokens(self, generator: &mut Generator) {
        generator.push(self.into())
    }
}
