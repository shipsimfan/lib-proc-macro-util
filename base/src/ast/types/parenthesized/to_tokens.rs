use crate::{ast::types::ParenthesizedType, Generator, ToTokens};

impl<'a> ToTokens for ParenthesizedType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#type.to_tokens(&mut generator.group_parenthesis());
    }
}
