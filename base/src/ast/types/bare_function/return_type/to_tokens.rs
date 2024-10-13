use crate::{ast::types::BareFunctionReturnType, Generator, ToTokens};

impl<'a> ToTokens for BareFunctionReturnType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.right_arrow.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
