use crate::{ast::types::ParenthesizedType, Generator, ToTokens};

impl ToTokens for ParenthesizedType {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#type.to_tokens(&mut generator.group_parenthesis());
    }
}
