use crate::{ast::Expression, Generator, ToTokens};

impl<'a> ToTokens for Expression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Expression::Literal(literal) => literal.to_tokens(generator),
        }
    }
}
