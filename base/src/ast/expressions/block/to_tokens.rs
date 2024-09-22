use crate::{ast::expressions::BlockExpression, Generator, ToTokens};

impl<'a> ToTokens for BlockExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let mut group = generator.group_brace();
        self.attributes.to_tokens(&mut group);
        self.statements.to_tokens(&mut group);
        self.end.to_tokens(&mut group);
    }
}
