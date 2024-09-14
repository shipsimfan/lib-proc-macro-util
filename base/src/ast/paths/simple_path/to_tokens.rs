use crate::{ast::SimplePath, Generator, ToTokens};

impl<'a> ToTokens for SimplePath<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.leading.to_tokens(generator);
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
    }
}
