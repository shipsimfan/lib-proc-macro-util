use crate::{ast::expressions::ArrayExpression, Generator, ToTokens};

impl<'a> ToTokens for ArrayExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.elements.to_tokens(&mut generator.group_bracket());
    }
}
