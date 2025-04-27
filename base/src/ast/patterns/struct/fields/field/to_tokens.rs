use crate::{ast::patterns::StructPatternField, Generator, ToTokens};

impl<'a> ToTokens for StructPatternField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.name.to_tokens(generator);
    }
}
