use crate::{ast::expressions::LazyBooleanExpression, Generator, ToTokens};

impl<'a> ToTokens for LazyBooleanExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.left.to_tokens(generator);
        self.operator.to_tokens(generator);
        self.right.to_tokens(generator);
    }
}
