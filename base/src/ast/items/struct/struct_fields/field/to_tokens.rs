use crate::{ast::items::StructField, Generator, ToTokens};

impl<'a> ToTokens for StructField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.visibility.to_tokens(generator);
        self.name.to_tokens(generator);
        self.colon.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
