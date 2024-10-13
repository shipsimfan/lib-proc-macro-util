use crate::{ast::Abi, Generator, ToTokens};

impl<'a> ToTokens for Abi<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.0.clone().to_tokens(generator);
    }
}
