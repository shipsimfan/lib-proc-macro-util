use crate::{ast::statements::LetStatement, Generator, ToTokens};

impl<'a> ToTokens for LetStatement<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.r#let.to_tokens(generator);
        self.pattern.to_tokens(generator);
        self.r#type.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
