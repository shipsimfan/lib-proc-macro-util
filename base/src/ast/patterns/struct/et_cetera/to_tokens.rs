use crate::{ast::patterns::StructPatternEtCetera, Generator, ToTokens};

impl<'a> ToTokens for StructPatternEtCetera<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.dots.to_tokens(generator);
    }
}
