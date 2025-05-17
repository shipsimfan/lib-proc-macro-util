use crate::{ast::expressions::TupleExpression, Generator, ToTokens};

impl<'a> ToTokens for TupleExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let generator = &mut generator.group_parenthesis();
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.last.to_tokens(generator);
    }
}
