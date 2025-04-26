use crate::ast::items::Module;
use std::borrow::Cow;

impl<'a> Module<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> Module<'static> {
        Module {
            r#unsafe: self.r#unsafe,
            r#mod: self.r#mod,
            identifier: match self.identifier {
                Cow::Owned(owned) => Cow::Owned(owned),
                Cow::Borrowed(borrowed) => Cow::Owned(borrowed.clone()),
            },
            body: self.body.into_static(),
        }
    }
}
