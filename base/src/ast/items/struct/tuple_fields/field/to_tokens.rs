use crate::{ast::items::TupleField, Generator, ToTokens};

impl<'a> ToTokens for TupleField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.visibility.to_tokens(generator);
        self.r#type.to_tokens(generator);
    }
}
