use crate::{ast::expressions::GroupedExpression, Generator, ToTokens};

impl<'a> ToTokens for GroupedExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression
            .to_tokens(&mut generator.group_parenthesis());
    }
}
