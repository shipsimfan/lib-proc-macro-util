use crate::ast::{items::FunctionQualifiers, Abi};

impl<'a> FunctionQualifiers<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionQualifiers<'static> {
        FunctionQualifiers {
            r#const: self.r#const,
            r#async: self.r#async,
            r#unsafe: self.r#unsafe,
            r#extern: self
                .r#extern
                .map(|(r#extern, abi)| (r#extern, abi.map(Abi::into_static))),
        }
    }
}
