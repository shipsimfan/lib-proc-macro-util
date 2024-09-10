use crate::{Generator, ToTokens};

impl<T: ToTokens + ?Sized> ToTokens for &T {
    fn to_tokens(&self, generator: &mut Generator) {
        (*self).to_tokens(generator)
    }
}

impl<'a, T: ToTokens + ToOwned<Owned = O>, O: ToTokens> ToTokens for std::borrow::Cow<'a, T> {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            std::borrow::Cow::Borrowed(value) => value.to_tokens(generator),
            std::borrow::Cow::Owned(value) => value.to_tokens(generator),
        }
    }
}
