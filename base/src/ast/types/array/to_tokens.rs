use crate::{ast::types::ArrayType, Generator, ToTokens};

impl<'a> ToTokens for ArrayType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let mut group = generator.group_bracket();

        self.r#type.to_tokens(&mut group);
        self.semi.to_tokens(&mut group);
        self.count.to_tokens(&mut group);
    }
}
