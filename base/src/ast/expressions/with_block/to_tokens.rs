use crate::{ast::ExpressionWithBlock, Generator, ToTokens};

impl<'a> ToTokens for ExpressionWithBlock<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
