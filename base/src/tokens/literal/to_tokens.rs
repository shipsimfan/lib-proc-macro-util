use crate::{tokens::Literal, Generator, ToTokens};

impl ToTokens for Literal {
    fn to_tokens(self, generator: &mut Generator) {
        generator.push(self.into())
    }
}
