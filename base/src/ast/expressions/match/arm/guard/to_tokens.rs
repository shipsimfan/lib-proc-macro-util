use crate::{ast::expressions::MatchArmGuard, Generator, ToTokens};

impl<'a> ToTokens for MatchArmGuard<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#if.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
