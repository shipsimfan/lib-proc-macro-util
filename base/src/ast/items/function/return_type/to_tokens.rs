use crate::{ast::items::FunctionReturnType, Generator, ToTokens};

impl<'a> ToTokens for FunctionReturnType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.arrow.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
