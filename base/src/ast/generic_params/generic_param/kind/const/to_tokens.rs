use crate::{ast::ConstParam, Generator, ToTokens};

impl<'a> ToTokens for ConstParam<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#const.to_tokens(generator);
        self.identifier.to_tokens(generator);
        self.colon.to_tokens(generator);
        self.r#type.to_tokens(generator);
        self.value.to_tokens(generator);
    }
}
