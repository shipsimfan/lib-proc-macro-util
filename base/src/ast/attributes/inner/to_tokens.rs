use crate::{ast::InnerAttribute, Generator, ToTokens};

impl<'a> ToTokens for InnerAttribute<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.hash.to_tokens(generator);
        self.exclamation.to_tokens(generator);
        self.attr.to_tokens(generator);
    }
}
