use crate::{ast::TypePath, Generator, ToTokens};

impl<'a> ToTokens for TypePath<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.leading.to_tokens(generator);
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
    }
}
