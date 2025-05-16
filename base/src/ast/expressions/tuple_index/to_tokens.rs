use crate::{ast::expressions::TupleIndexExpression, Generator, ToTokens};

impl<'a> ToTokens for TupleIndexExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.index.to_tokens(generator);
    }
}
