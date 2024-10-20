use crate::{ast::items::FunctionBody, Generator, ToTokens};

impl<'a> ToTokens for FunctionBody<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            FunctionBody::Expression(expression) => expression.to_tokens(generator),
            FunctionBody::Semi(semi) => semi.to_tokens(generator),
        }
    }
}
