use crate::{ast::Item, Generator, ToTokens};

impl<'a> ToTokens for Item<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
