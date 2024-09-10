use crate::{Generator, ToTokens};

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, generator: &mut Generator) {
        if let Some(t) = self {
            t.to_tokens(generator)
        }
    }
}
