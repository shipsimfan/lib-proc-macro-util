use crate::{ast::expressions::AssignmentExpression, Generator, ToTokens};

impl<'a> ToTokens for AssignmentExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.left.to_tokens(generator);
        self.equals.to_tokens(generator);
        self.right.to_tokens(generator);
    }
}
