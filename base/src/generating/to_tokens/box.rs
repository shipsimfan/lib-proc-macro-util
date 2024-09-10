use crate::{Generator, ToTokens};

impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, generator: &mut Generator) {
        self.as_ref().to_tokens(generator)
    }
}
