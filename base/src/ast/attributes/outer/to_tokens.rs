use crate::{ast::OuterAttribute, Generator, ToTokens};

impl<'a> ToTokens for OuterAttribute<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.hash.to_tokens(generator);
        self.attr.to_tokens(generator);
    }
}
