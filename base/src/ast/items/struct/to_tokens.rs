use crate::{ast::items::Struct, Generator, ToTokens};

impl<'a> ToTokens for Struct<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#struct.to_tokens(generator);
        self.name.to_tokens(generator);
        self.generic_params.to_tokens(generator);
        self.body.to_tokens(generator);
    }
}
