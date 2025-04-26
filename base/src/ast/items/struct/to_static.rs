use crate::ast::{items::Struct, GenericParams};
use std::borrow::Cow;

impl<'a> Struct<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Struct<'static> {
        Struct {
            r#struct: self.r#struct,
            name: Cow::Owned(match self.name {
                Cow::Owned(owned) => owned,
                Cow::Borrowed(borrowed) => borrowed.clone(),
            }),
            generic_params: self.generic_params.map(GenericParams::into_static),
            body: self.body.into_static(),
        }
    }
}
