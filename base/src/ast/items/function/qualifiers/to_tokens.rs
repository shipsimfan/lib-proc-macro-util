use crate::{ast::items::FunctionQualifiers, Generator, ToTokens};

impl<'a> ToTokens for FunctionQualifiers<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#const.to_tokens(generator);
        self.r#async.to_tokens(generator);
        self.r#unsafe.to_tokens(generator);
        self.r#extern.to_tokens(generator);
    }
}
