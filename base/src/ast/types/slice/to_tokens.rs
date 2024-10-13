use crate::{ast::types::SliceType, Generator, ToTokens};

impl<'a> ToTokens for SliceType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#type.to_tokens(&mut generator.group_bracket());
    }
}
