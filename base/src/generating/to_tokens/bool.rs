use crate::{tokens::Identifier, Generator, ToTokens};

impl ToTokens for bool {
    fn to_tokens(self, generator: &mut Generator) {
        Identifier::new(if self { "true" } else { "false" }).to_tokens(generator);
    }
}
