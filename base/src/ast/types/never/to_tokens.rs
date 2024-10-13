use crate::{ast::types::NeverType, Generator, ToTokens};

impl ToTokens for NeverType {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.to_tokens(generator);
    }
}
