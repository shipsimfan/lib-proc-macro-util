use crate::{ast::expressions::ClosureExpression, Generator, ToTokens};

impl<'a> ToTokens for ClosureExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#async.to_tokens(generator);
        self.r#move.to_tokens(generator);
        self.leading.to_tokens(generator);
        self.parameters.to_tokens(generator);
        self.trailing.to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
