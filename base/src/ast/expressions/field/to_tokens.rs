use crate::{ast::expressions::FieldExpression, Generator, ToTokens};

impl<'a> ToTokens for FieldExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.identifier.to_tokens(generator);
    }
}
