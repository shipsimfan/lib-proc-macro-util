use crate::{ast::patterns::RangeToInclusivePattern, Generator, ToTokens};

impl<'a> ToTokens for RangeToInclusivePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.dots.to_tokens(generator);
        self.upper.to_tokens(generator);
    }
}
