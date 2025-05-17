use crate::{ast::expressions::ClosureParam, Generator, ToTokens};

impl<'a> ToTokens for ClosureParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.pattern.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
