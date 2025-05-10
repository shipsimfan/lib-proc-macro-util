use crate::{ast::expressions::ComparisonExpression, Generator, ToTokens};

impl<'a> ToTokens for ComparisonExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.left.to_tokens(generator);
        self.operator.to_tokens(generator);
        self.right.to_tokens(generator);
    }
}
