use crate::{tokens::Identifier, Generator, ToTokens};

impl ToTokens for Identifier {
    fn to_tokens(self, generator: &mut Generator) {
        generator.push(self.into())
    }
}
