use crate::{ast::patterns::PathPattern, Generator, ToTokens};

impl<'a> ToTokens for PathPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
    }
}
