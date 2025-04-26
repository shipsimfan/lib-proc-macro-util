use crate::{ast::patterns::IdentifierPattern, Generator, ToTokens};

impl<'a> ToTokens for IdentifierPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#ref.to_tokens(generator);
        self.r#mut.to_tokens(generator);
        self.identifier.to_tokens(generator);
        self.pattern.to_tokens(generator);
    }
}
