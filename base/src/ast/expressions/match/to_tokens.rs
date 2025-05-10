use crate::{ast::expressions::MatchExpression, Generator, ToTokens};

impl<'a> ToTokens for MatchExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#match.to_tokens(generator);
        self.scrutinee.to_tokens(generator);

        let mut generator = generator.group_brace();
        self.attributes.to_tokens(&mut generator);
        self.arms.to_tokens(&mut generator);
    }
}
