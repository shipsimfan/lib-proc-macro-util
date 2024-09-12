use crate::{tokens::Literal, Generator, ToTokens};

impl ToTokens for char {
    fn to_tokens(self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}
