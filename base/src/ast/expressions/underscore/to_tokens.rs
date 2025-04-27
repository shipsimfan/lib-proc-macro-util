use crate::{ast::expressions::UnderscoreExpression, Generator, ToTokens};

impl ToTokens for UnderscoreExpression {
    fn to_tokens(self, generator: &mut Generator) {
        self.underscore.to_tokens(generator);
    }
}
