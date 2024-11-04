use crate::{ast::items::StructField, Generator, ToTokens};
use std::borrow::Cow;

impl<'a> ToTokens for StructField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.visibility.to_tokens(generator);
        match self.name {
            Cow::Borrowed(name) => name.to_owned(),
            Cow::Owned(name) => name,
        }
        .to_tokens(generator);
        self.colon.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
