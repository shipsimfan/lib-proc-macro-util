use crate::{ast::statements::ExpressionStatement, Generator, ToTokens};

impl<'a> ToTokens for ExpressionStatement<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ExpressionStatement::WithoutBlock(expression, comma) => {
                expression.to_tokens(generator);
                comma.to_tokens(generator);
            }
            ExpressionStatement::WithBlock(expression, comma) => {
                expression.to_tokens(generator);
                comma.to_tokens(generator);
            }
        }
    }
}
