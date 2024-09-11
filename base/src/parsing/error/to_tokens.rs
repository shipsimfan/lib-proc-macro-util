use crate::{Error, Generator, ToTokens};

impl ToTokens for Error {
    fn to_tokens(self, generator: &mut Generator) {
        for error in self.errors {
            error.to_tokens(generator);
        }
    }
}
