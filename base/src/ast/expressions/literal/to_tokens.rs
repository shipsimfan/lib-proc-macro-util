use crate::{ast::expressions::LiteralExpression, Generator, ToTokens};

impl<'a> ToTokens for LiteralExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            LiteralExpression::Literal(literal) => literal.clone().to_tokens(generator),
            LiteralExpression::OwnedLiteral(literal) => literal.to_tokens(generator),
            LiteralExpression::True(r#true) => r#true.to_tokens(generator),
            LiteralExpression::False(r#false) => r#false.to_tokens(generator),
        }
    }
}
