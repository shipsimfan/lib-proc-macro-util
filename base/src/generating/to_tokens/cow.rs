use crate::{Generator, ToTokens};
use std::borrow::Cow;

impl<'a, T: ToTokens + Clone> ToTokens for Cow<'a, T> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Cow::Borrowed(borrowed) => borrowed.clone(),
            Cow::Owned(owned) => owned,
        }
        .to_tokens(generator);
    }
}
