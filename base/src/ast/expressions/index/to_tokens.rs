use crate::{ast::expressions::IndexExpression, Generator, ToTokens};

impl<'a> ToTokens for IndexExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.index.to_tokens(&mut generator.group_bracket());
    }
}
