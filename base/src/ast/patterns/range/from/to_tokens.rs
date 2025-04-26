use crate::{ast::patterns::RangeFromPattern, Generator, ToTokens};

impl<'a> ToTokens for RangeFromPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.lower.to_tokens(generator);
        self.dots.to_tokens(generator);
    }
}
