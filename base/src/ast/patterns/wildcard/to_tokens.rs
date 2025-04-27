use crate::{ast::patterns::WildcardPattern, Generator, ToTokens};

impl ToTokens for WildcardPattern {
    fn to_tokens(self, generator: &mut Generator) {
        self.underscore.to_tokens(generator);
    }
}
