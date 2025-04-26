use crate::{ast::Expression, Generator, ToTokens};

impl<'a> ToTokens for Expression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Expression::WithBlock(expression) => expression.to_tokens(generator),
            Expression::WithoutBlock(expression) => expression.to_tokens(generator),
        }
    }
}
