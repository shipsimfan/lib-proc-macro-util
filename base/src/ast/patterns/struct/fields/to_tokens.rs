use crate::{ast::patterns::StructPatternFields, Generator, ToTokens};

impl<'a> ToTokens for StructPatternFields<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
    }
}
