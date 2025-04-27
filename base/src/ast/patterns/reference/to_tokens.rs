use crate::{ast::patterns::ReferencePattern, Generator, ToTokens};

impl<'a> ToTokens for ReferencePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.ampersand.to_tokens(generator);
        self.r#mut.to_tokens(generator);
        self.pattern.to_tokens(generator);
    }
}
