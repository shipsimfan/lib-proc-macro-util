use crate::{tokens::Literal, Generator, ToTokens};

impl ToTokens for str {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for String {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::ffi::OsStr {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::ffi::OsString {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::ffi::CStr {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::ffi::CString {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::path::Path {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}

impl ToTokens for std::path::PathBuf {
    fn to_tokens(&self, generator: &mut Generator) {
        Literal::new(self).to_tokens(generator)
    }
}
