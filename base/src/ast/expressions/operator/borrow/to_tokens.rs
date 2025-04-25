use crate::{ast::expressions::BorrowExpression, Generator, ToTokens};

impl<'a> ToTokens for BorrowExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.ampersand.to_tokens(generator);
        self.mutable.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
