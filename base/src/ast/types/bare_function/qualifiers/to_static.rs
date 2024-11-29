use crate::ast::{types::FunctionTypeQualifiers, Abi};

impl<'a> FunctionTypeQualifiers<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> FunctionTypeQualifiers<'static> {
        FunctionTypeQualifiers {
            r#unsafe: self.r#unsafe,
            r#extern: self
                .r#extern
                .map(|(r#extern, abi)| (r#extern, abi.map(Abi::into_static))),
        }
    }
}
