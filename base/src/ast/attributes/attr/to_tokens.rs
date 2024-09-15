use crate::{ast::Attr, Generator, ToTokens};

impl<'a> ToTokens for Attr<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
        self.input.to_tokens(generator);
    }
}
