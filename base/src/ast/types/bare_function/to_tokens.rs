use crate::{ast::types::BareFunctionType, Generator, ToTokens};

impl<'a> ToTokens for BareFunctionType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.for_lifetimes.to_tokens(generator);
        self.qualifiers.to_tokens(generator);
        self.r#fn.to_tokens(generator);
        self.parameters
            .to_tokens(&mut generator.group_parenthesis());
        self.return_type.to_tokens(generator);
    }
}
