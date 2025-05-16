use crate::{ast::expressions::TypeCastExpression, Generator, ToTokens};

impl<'a> ToTokens for TypeCastExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.r#as.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
