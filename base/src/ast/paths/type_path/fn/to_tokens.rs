use crate::{ast::TypePathFn, Generator, ToTokens};

impl ToTokens for TypePathFn {
    fn to_tokens(self, generator: &mut Generator) {
        self.inputs.to_tokens(&mut generator.group_parenthesis());
        self.r#return.to_tokens(generator);
    }
}
