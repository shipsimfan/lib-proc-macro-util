use crate::{ast::patterns::RestPattern, Generator, ToTokens};

impl ToTokens for RestPattern {
    fn to_tokens(self, generator: &mut Generator) {
        self.dots.to_tokens(generator);
    }
}
