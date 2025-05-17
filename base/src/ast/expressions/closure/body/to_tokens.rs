use crate::{ast::expressions::ClosureBody, Generator, ToTokens};

impl<'a> ToTokens for ClosureBody<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ClosureBody::Expression(expression) => expression.to_tokens(generator),
            ClosureBody::ReturnType {
                arrow,
                r#type,
                expression,
            } => {
                arrow.to_tokens(generator);
                r#type.to_tokens(generator);
                expression.to_tokens(generator);
            }
        }
    }
}
