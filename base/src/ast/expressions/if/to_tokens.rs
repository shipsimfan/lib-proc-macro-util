use crate::{ast::expressions::IfExpression, Generator, ToTokens};

impl<'a> ToTokens for IfExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        todo!()
    }
}
