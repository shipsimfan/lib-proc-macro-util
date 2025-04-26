use crate::{ast::patterns::RangeInclusivePattern, Generator, ToTokens};

impl<'a> ToTokens for RangeInclusivePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.lower.to_tokens(generator);
        self.dots.to_tokens(generator);
        self.upper.to_tokens(generator);
    }
}
