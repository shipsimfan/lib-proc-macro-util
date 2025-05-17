use crate::{ast::expressions::RangeExpression, Generator, ToTokens};

impl<'a> ToTokens for RangeExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.lower.to_tokens(generator);
        self.operator.to_tokens(generator);
        self.upper.to_tokens(generator);
    }
}
